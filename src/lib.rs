use std::error::Error;

use crate::http_engine::HttpServer;

pub mod http_engine;
pub mod server;

pub struct CorpoMCP;

impl CorpoMCP {
    pub async fn init() -> Result<(), Box<dyn Error>> {
        //TODO: remove burn http
        let server = HttpServer::new("127.0.0.1", 6450).await;

        println!("Started server");

        Ok(server.run().await?)
    }
}
