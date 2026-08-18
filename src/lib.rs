use crate::http_engine::HttpServer;

pub mod http_engine;
pub mod server;

pub struct CorpoMCP;

impl CorpoMCP {
    pub async fn init() {
        //TODO: remove burn http
        let server = HttpServer::new("127.0.0.1", 8080).await;

        println!("Started server");

        server.run().await;
    }
}
