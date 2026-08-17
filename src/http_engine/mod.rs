use async_std::net::TcpListener;
use futures::StreamExt;
use std::format;

struct HttpServer {
    host: String,
    port: usize,
    listener: TcpListener,
}

impl HttpServer {
    pub async fn new(host: String, port: usize) -> Self {
        let listener = TcpListener::bind(format!("{host}:{port}"))
            .await
            .expect("Couldn't bind to address");

        Self {
            host,
            port,
            listener,
        }
    }

    pub async fn run(self) {
        self.listener
            .incoming()
            .for_each_concurrent(None, |tcpstream| async move {
                //TODO: add simple handler
            });
    }
}
