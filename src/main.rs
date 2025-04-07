#[tokio::main]
async fn main() -> anyhow::Result<()> {
    weather_rmcp::server().await?;
    Ok(())
}
