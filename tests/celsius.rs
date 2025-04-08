use anyhow::Result;
use weather_rmcp::weather::{Point, WeatherClient};

#[tokio::test]
async fn test_celsius() -> Result<()> {
    // 创建天气客户端
    let weather = WeatherClient::default();

    // 设置位置坐标（纽约）
    let point = Point {
        latitude: 40.7128,
        longitude: -74.0060,
    };

    println!("正在获取纽约的天气预报...");

    // 获取天气预报
    match weather.get_weather_forecast(&point).await {
        Ok(forecast) => {
            // 展示所有天气周期的摄氏度温度
            println!("纽约未来天气预报 (摄氏度): ");

            // 先计算最高和最低温度
            let mut max_temp = None;
            let mut min_temp = None;

            for period in &forecast.properties.periods {
                let celsius = period.celsius();

                // 更新最高温度
                max_temp = match max_temp {
                    None => Some(celsius),
                    Some(t) if celsius > t => Some(celsius),
                    Some(t) => Some(t),
                };

                // 更新最低温度
                min_temp = match min_temp {
                    None => Some(celsius),
                    Some(t) if celsius < t => Some(celsius),
                    Some(t) => Some(t),
                };

                // 直接使用摄氏度温度
                println!(
                    "• {}: {:.1} °C, {}",
                    period.name, celsius, period.short_forecast
                );
            }

            // 显示最高和最低温度
            if let Some(max) = max_temp {
                println!("\n最高温度: {:.1} °C", max);
            }

            if let Some(min) = min_temp {
                println!("最低温度: {:.1} °C", min);
            }
        }
        Err(e) => {
            eprintln!("获取天气预报失败: {:?}", e);
        }
    }

    Ok(())
}
