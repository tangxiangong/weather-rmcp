use anyhow::Result;
use weather_rmcp::weather::{Point, WeatherClient};

#[tokio::test]
async fn test_weather() -> Result<()> {
    // 创建天气客户端
    let weather = WeatherClient::default();

    // 设置位置坐标（旧金山）
    let point = Point {
        latitude: 37.7749,
        longitude: -122.4194,
    };

    // 获取天气预报
    println!("正在获取旧金山的天气预报...");

    // 使用 info 方法直接获取并显示天气信息（包括摄氏度）
    weather.info(&point).await?;

    println!("\n成功获取天气预报！");

    Ok(())
}
