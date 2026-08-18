use corpo_metrics::CorpoMCP;

#[async_std::main]
async fn main() {
    CorpoMCP::init().await
}
