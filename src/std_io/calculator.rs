use crate::utils;
use rmcp::{
    ServerHandler,
    model::{ServerCapabilities, ServerInfo},
    schemars, tool,
};

#[derive(Debug, Clone)]
pub struct Calculator;

#[tool(tool_box)]
impl Calculator {
    #[tool(description = "Calculate the sum of two numbers")]
    fn sum(
        &self,
        #[tool(param)]
        #[schemars(description = "the left hand side number")]
        lhs: f64,
        #[tool(param)]
        #[schemars(description = "the right hand side number")]
        rhs: f64,
    ) -> String {
        utils::sum(lhs, rhs).to_string()
    }

    #[tool(description = "Calculate the sub of two numbers")]
    fn sub(
        &self,
        #[tool(param)]
        #[schemars(description = "the left hand side number")]
        lhs: f64,
        #[tool(param)]
        #[schemars(description = "the right hand side number")]
        rhs: f64,
    ) -> String {
        utils::sub(lhs, rhs).to_string()
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
