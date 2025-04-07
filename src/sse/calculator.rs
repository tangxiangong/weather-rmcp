use crate::utils;
use rmcp::{
    ServerHandler,
    handler::server::wrapper::Json,
    model::{ServerCapabilities, ServerInfo},
    schemars, tool,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SumRequest {
    #[schemars(description = "the left hand side number")]
    pub lhs: f64,
    #[schemars(description = "the right hand side number")]
    pub rhs: f64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SubRequest {
    #[schemars(description = "the left hand side number")]
    pub lhs: f64,
    #[schemars(description = "the right hand side number")]
    pub rhs: f64,
}

#[derive(Debug, Clone)]
pub struct Calculator;

#[tool(tool_box)]
impl Calculator {
    #[tool(description = "Calculate the sum of two numbers")]
    pub fn sum(&self, #[tool(aggr)] SumRequest { lhs, rhs }: SumRequest) -> Json<f64> {
        Json(utils::sum(lhs, rhs))
    }

    #[tool(description = "Calculate the sub of two numbers")]
    pub fn sub(&self, #[tool(aggr)] SubRequest { lhs, rhs }: SubRequest) -> Json<f64> {
        Json(utils::sub(lhs, rhs))
    }
}

#[tool(tool_box)]
impl ServerHandler for Calculator {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A simple calculator".to_string()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
