use async_std::net::{TcpListener, TcpStream};
use futures::{AsyncReadExt, StreamExt};
use serde_json::Value;
use std::{error::Error, fmt::Display, format, io::Read, ptr::eq};

use http::{Request as HttpRequest, Response as HttpResponse};
use serde::{Deserialize, de};

use crate::server::primitives::McpServer;

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
        // init for the request
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        // init response in case of error, just basic handleling
        let mut resp = HttpResponse::builder().status(200);

        // getting the raw stream
        let content = &mut String::new();
        stream.read_to_string(content).await.unwrap();

        // for getting the body
        if let Ok(httparse::Status::Complete(header_len)) =
            req.parse(content.clone().into_bytes().as_slice())
        {
            // checking for Content-type
            if !HttpServer::check_content_type_headers(req.headers) {
                return Err(Box::new(HttpServerErrors::IncorrectContentTypeHeader));
            }

            println!("correct headers");

            let body = &content[header_len..];
            let body = HttpServer::parse_body_into_json(body)?;

            //TODO: do the mcp server call in here
            match McpServer::handle_response(body).await {
                Ok(payload) => {
                    println!("{:?}", payload);
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
}
