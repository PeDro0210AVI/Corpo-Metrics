use async_std::{
    io::WriteExt,
    net::{TcpListener, TcpStream},
};
use futures::{AsyncReadExt, StreamExt};
use serde_json::Value;
use std::{error::Error, fmt::Display, format, io::Read, ptr::eq};

use http::{Request as HttpRequest, Response as HttpResponse};
use serde::{Deserialize, de};

use crate::server::primitives::{McpJson, McpServer};

const CONTENT_TYPE_RAW: &str = "Content-Type";
const APPLICATION_JSON_RAW: &str = "application/json";

pub struct HttpServer {
    host: Box<str>,
    port: usize,
    listener: TcpListener,
}

#[derive(Debug)]
pub enum HttpServerErrors {
    IncorrectContentTypeHeader,
}

impl Display for HttpServerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for HttpServerErrors {}

//http server just for managing json content types for the MCP
impl HttpServer {
    pub async fn new(host: &str, port: usize) -> Self {
        let listener = TcpListener::bind(format!("{host}:{port}"))
            .await
            .expect("Couldn't bind to address");

        Self {
            host: host.into(),
            port,
            listener,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        Ok(self
            .listener
            .incoming()
            .for_each_concurrent(None, |tcpstream| async move {
                let tcpstream = match tcpstream {
                    Ok(stream) => stream,
                    Err(err) => {
                        println!("Error: couldn't get stream");
                        return;
                    }
                };
                HttpServer::handle_stream(tcpstream).await;
            })
            .await)
    }

    // just for managing json request, in other cases it won't work
    // TODO: soooner or later send a generic response
    async fn handle_stream(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        let resp = HttpResponse::builder().status(200);

        // Read until we have headers
        let mut content = String::new();
        let mut partial = [0u8; 1];

        // Read byte by byte until we find \r\n\r\n (end of headers)
        while !content.ends_with("\r\n\r\n") && content.len() < 8192 {
            stream.read_exact(&mut partial).await?;
            content.push(partial[0] as char);
        }

        if let Ok(httparse::Status::Complete(_)) = req.parse(content.as_bytes()) {
            if !HttpServer::check_content_type_headers(req.headers) {
                return Err(Box::new(HttpServerErrors::IncorrectContentTypeHeader));
            }

            let content_length: usize = req
                .headers
                .iter()
                .find(|h| h.name.eq_ignore_ascii_case("content-length"))
                .and_then(|h| std::str::from_utf8(h.value).ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);

            let mut body_buf = vec![0u8; content_length];
            stream.read_exact(&mut body_buf).await?;
            let body_str = std::str::from_utf8(&body_buf)?;

            let body = HttpServer::parse_body_into_json(body_str)?;

            match McpServer::handle_response(body).await {
                Ok(payload) => {
                    let response = resp
                        .header("Content-Type", "application/json")
                        .body(payload)?;
                    HttpServer::send_response(&mut stream, &response).await?;
                }
                Err(err) => {
                    println!("Error {:?}", err);
                }
            };
        }

        Ok(())
    }
    fn check_content_type_headers(headers: &[httparse::Header]) -> bool {
        for header in headers {
            // parsing in it cause we just searching for APPLICATION_JSON_RAW
            let string_val = str::from_utf8(header.value).unwrap_or_default();
            if header.name == CONTENT_TYPE_RAW.to_string()
                && string_val == APPLICATION_JSON_RAW.to_string()
            {
                return true;
            }
        }
        return false;
    }

    fn parse_body_into_json(raw_payload: &str) -> Result<Value, serde_json::Error> {
        let mut deserializer = serde_json::Deserializer::from_str(raw_payload);
        let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
        let value = Value::deserialize(deserializer);
        value
    }

    async fn send_response(
        stream: &mut TcpStream,
        response: &HttpResponse<McpJson>,
    ) -> Result<(), Box<dyn Error>> {
        let status = response.status();
        let headers = response.headers();
        let body = response.clone().into_body();

        let mut response_bytes = String::new();
        response_bytes.push_str(&format!(
            "HTTP/1.1 {} {}\r\n",
            status.as_u16(),
            status.canonical_reason().unwrap_or("Unknown")
        ));

        // Add headers
        for (name, value) in headers.iter() {
            let header_name = name.as_str();
            let header_value = value.to_str().unwrap_or("");
            response_bytes.push_str(&format!("{}: {}\r\n", header_name, header_value));
        }
        response_bytes.push_str("\r\n");

        stream.write_all(response_bytes.as_bytes()).await?;

        let body_bytes = serde_json::to_vec(&body)?;
        stream.write_all(&body_bytes).await?;

        Ok(())
    }
}
