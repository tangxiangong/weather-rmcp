use crate::weather::{Point, WeatherClient};
use rmcp::{
    ServerHandler, ServiceExt,
    model::{ServerCapabilities, ServerInfo},
    schemars, tool,
    transport::stdio,
};
use tracing_subscriber::{self, EnvFilter};

#[derive(Debug, Clone)]
pub struct Weather;

#[tool(tool_box)]
impl Weather {
    #[tool(description = "Get the weather information of a specific location")]
    async fn get_weather_info(
        &self,
        #[tool(param)]
        #[schemars(description = "the latitude of the location")]
        latitude: f64,
        #[tool(param)]
        #[schemars(description = "the longitude of the location")]
        longitude: f64,
    ) -> String {
        let point = Point {
            latitude,
            longitude,
        };
        let weather = WeatherClient::default();
        weather
            .info(&point)
            .await
            .unwrap_or("Failed to get weather information".to_string())
    }
}

#[tool(tool_box)]
impl ServerHandler for Weather {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A simple weather information provider".to_string()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

pub async fn server() -> anyhow::Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    // Create an instance of our counter router
    let service = Weather.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
