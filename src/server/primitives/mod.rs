use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};

// stateless
pub struct McpServer;

// JsonRPC response
#[derive(Serialize, Deserialize, Debug)]
struct JsonRPC {
    jsonrpc: String,
    id: u32,
    method: String,
}

// wrapper
impl McpServer {
    pub fn handle_response(request: Value) {
        let json_rpc_request = McpServer::parse_request(request);

        println!("{json_rpc_request:?}");

        //TODO: see with the calls
    }

    fn parse_request(request: Value) -> Result<JsonRPC, Error> {
        serde_json::from_value::<JsonRPC>(request)
    }

    fn mcp_call() {}

    fn mcp_list() {}
}
