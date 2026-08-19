use std::{error::Error, fmt::Display};

use async_std::{
    fs::{File, OpenOptions},
    io::ReadExt,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const MCP_TOOLS_LIST_RAW: &str = "tools/list";
const MCP_TOOLS_CALL_RAW: &str = "tools/call";

// stateless
pub struct McpServer;

// JsonRPC response
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct McpJson {
    jsonrpc: String,
    id: Option<u32>,
    method: Option<String>,
    params: Option<Value>,
    result: Option<Value>,
}

#[derive(Debug)]
pub enum McpServerError {
    NotMethodSupply,
    CouldntFullFilledResponse,
}

impl Display for McpServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for McpServerError {}

// wrapper
impl McpServer {
    pub async fn handle_response(request: Value) -> Result<McpJson, Box<dyn Error>> {
        let json_rpc_request = serde_json::from_value::<McpJson>(request)?;

        if let None = json_rpc_request.method {
            return Err(Box::new(McpServerError::NotMethodSupply));
        }

        // the unwrap should be safe here
        match json_rpc_request.method.unwrap().as_str() {
            MCP_TOOLS_CALL_RAW => {
                todo!()
            }
            MCP_TOOLS_LIST_RAW => McpServer::mcp_list().await,
            _ => {
                return Err(Box::new(McpServerError::CouldntFullFilledResponse));
            }
        }
    }

    fn mcp_call() -> Result<McpJson, Box<dyn Error>> {
        todo!()
    }

    async fn mcp_list() -> Result<McpJson, Box<dyn Error>> {
        // grabbing the tools_list json
        let mut file = File::open("data/tools_list.json").await?;

        let content = &mut String::new();

        file.read_to_string(content).await?;

        let mut deserializer = serde_json::Deserializer::from_str(&content);
        let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
        let value = Value::deserialize(deserializer)?;

        Ok(McpJson {
            jsonrpc: "2.0".to_string(),
            result: Some(value),
            ..Default::default()
        })
    }
}
