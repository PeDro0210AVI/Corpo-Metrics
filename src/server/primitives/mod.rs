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
    params: Option<MCPCallparams>,
    result: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MCPCallparams {
    name: String,
    arguments: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResultType {
    complete,
    inputRequired,
    task,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentType {
    text,
    // just adding text for the moment
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentText {
    r#type: ContentType,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct McpToolListNames {
    names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct McpPayload {
    resultType: ResultType,
    content: Vec<ContentText>,
}

#[derive(Debug)]
pub enum McpServerError {
    NotMethodSupply,
    CouldntFullFilledResponse,
    CouldntGetCallArguments,
}

impl Display for McpServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for McpServerError {}

// wrapper
impl McpServer {
    pub async fn handle_response(payload: Value) -> Result<McpJson, Box<dyn Error>> {
        let json_rpc_request = serde_json::from_value::<McpJson>(payload)?;

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

    //TODO: all opening files things load them at startup
    async fn mcp_call(json_rpc_request: McpJson) -> Result<McpJson, Box<dyn Error>> {
        if let None = json_rpc_request.params {
            return Err(Box::new(McpServerError::CouldntFullFilledResponse));
        };

        let mcp_params = json_rpc_request.params.unwrap();

        let result: Result<McpPayload, McpServerError> = match mcp_params.name.as_str() {
            "get_sales_metrics" => {
                todo!()
            }
            "get_top_products" => {
                todo!()
            }
            "get_financial_status" => {
                todo!()
            }
            "get_customer_metrics" => {
                todo!()
            }
            "get_project_status" => {
                todo!()
            }
            "get_team_metrics" => {
                todo!()
            }
            "get_product_health" => {
                todo!()
            }
            "generate_executive_summary" => {
                todo!()
            }
            "get_anomalies" => {
                todo!()
            }
            "get_inventory_levels" => {
                todo!()
            }
            _ => Err(McpServerError::CouldntGetCallArguments),
        };

        let result = Some(serde_json::to_value(result?)?);

        Ok(McpJson {
            jsonrpc: "2.0".to_string(),
            result,
            ..Default::default()
        })
    }

    async fn mcp_list() -> Result<McpJson, Box<dyn Error>> {
        // grabbing the tools_list json
        let mut file = File::open("data/mcp/tools_list.json").await?;

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
