use anyhow::Result;
use reqwest::{Client, header::HeaderMap};
use serde::Deserialize;

const API_BASE_URL: &str = "https://api.weather.gov";
const USER_AGENT: &str = "weather-app/1.0";

pub struct WeatherClient {
    client: Client,
}

impl Default for WeatherClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub properties: Properties,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub forecast: String,
    #[serde(rename = "forecastHourly")]
    pub forecast_hourly: String,
    #[serde(rename = "relativeLocation")]
    pub relative_location: RelativeLocation,
    #[serde(rename = "gridId")]
    pub grid_id: String,
    #[serde(rename = "gridX")]
    pub grid_x: i32,
    #[serde(rename = "gridY")]
    pub grid_y: i32,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

#[derive(Debug, Deserialize)]
pub struct RelativeLocation {
    pub properties: LocationProperties,
}

#[derive(Debug, Deserialize)]
pub struct LocationProperties {
    pub city: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub properties: ForecastProperties,
}

#[derive(Debug, Deserialize)]
pub struct ForecastProperties {
    pub periods: Vec<ForecastPeriod>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastPeriod {
    pub number: i32,
    pub name: String,
    pub temperature: i32,
    #[serde(rename = "temperatureUnit")]
    pub temperature_unit: String,
    #[serde(rename = "windSpeed")]
    pub wind_speed: String,
    #[serde(rename = "windDirection")]
    pub wind_direction: String,
    #[serde(rename = "shortForecast")]
    pub short_forecast: String,
    #[serde(rename = "detailedForecast")]
    pub detailed_forecast: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(skip, default = "Default::default")]
    pub temperature_celsius: Option<f64>,
}

impl ForecastPeriod {
    // 计算并获取摄氏度温度
    pub fn celsius(&self) -> f64 {
        self.temperature_celsius.unwrap_or_else(|| {
            if self.temperature_unit == "F" {
                fahrenheit_to_celsius(self.temperature as f64)
            } else {
                self.temperature as f64
            }
        })
    }

    // 格式化显示摄氏度温度，保留一位小数
    pub fn celsius_formatted(&self) -> String {
        format!("{:.1}", self.celsius())
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

impl WeatherClient {
    pub async fn info(&self, point: &Point) -> Result<String> {
        let mut information = String::new();
        match self.get_weather_forecast(point).await {
            Ok(forecast) => {
                information.push_str(&format!(
                    "天气预报更新时间: {:?}\n",
                    forecast.properties.update_time
                ));

                for period in forecast.properties.periods {
                    information.push_str("=====================\n");
                    information.push_str(&format!("时段: {}\n", period.name));

                    // 温度单位通常是 F (华氏度)
                    if period.temperature_unit == "F" {
                        information.push_str(&format!(
                            "温度: {} °F / {} °C",
                            period.temperature,
                            period.celsius_formatted()
                        ));
                    } else {
                        information.push_str(&format!(
                            "温度: {} {}",
                            period.temperature, period.temperature_unit
                        ));
                    }

                    information.push_str(&format!(
                        "风向: {} 风速: {}",
                        period.wind_direction, period.wind_speed
                    ));
                    information.push_str(&format!("简要预报: {}\n", period.short_forecast));
                    information.push_str(&format!("详细预报: {}\n", period.detailed_forecast));
                    information.push_str(&format!("开始时间: {}\n", period.start_time));
                    information.push_str(&format!("结束时间: {}\n", period.end_time));
                }
            }
            Err(e) => println!("获取天气预报失败: {:?}", e),
        }
        Ok(information)
    }

    pub async fn get_point_data(&self, point: &Point) -> Result<WeatherResponse> {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", USER_AGENT.parse()?);
        headers.insert("Accept", "application/geo+json".parse()?);

        let url = format!(
            "{}/points/{},{}",
            API_BASE_URL, point.latitude, point.longitude
        );
        let response = self.client.get(url).headers(headers).send().await?;

        let point_data = response.json::<WeatherResponse>().await?;
        Ok(point_data)
    }

    pub async fn get_weather_forecast(&self, point: &Point) -> Result<ForecastResponse> {
        let point_data = self.get_point_data(point).await?;

        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", USER_AGENT.parse()?);
        headers.insert("Accept", "application/geo+json".parse()?);

        let response = self
            .client
            .get(&point_data.properties.forecast)
            .headers(headers)
            .send()
            .await?;

        let mut forecast_data = response.json::<ForecastResponse>().await?;

        // 计算并设置摄氏度温度
        for period in &mut forecast_data.properties.periods {
            if period.temperature_unit == "F" {
                period.temperature_celsius = Some(fahrenheit_to_celsius(period.temperature as f64));
            } else {
                period.temperature_celsius = Some(period.temperature as f64);
            }
        }

        Ok(forecast_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_point_data() {
        let weather = WeatherClient::default();
        let point = Point {
            latitude: 37.7749,
            longitude: -122.4194,
        };
        let point_data = weather.get_point_data(&point).await.unwrap();
        println!("Forecast URL: {}", point_data.properties.forecast);
        println!(
            "Hourly Forecast URL: {}",
            point_data.properties.forecast_hourly
        );
        println!(
            "Location: {}, {}",
            point_data.properties.relative_location.properties.city,
            point_data.properties.relative_location.properties.state
        );
    }

    #[tokio::test]
    async fn test_get_weather_forecast() {
        let weather = WeatherClient::default();
        let point = Point {
            latitude: 37.7749,
            longitude: -122.4194,
        };

        match weather.get_weather_forecast(&point).await {
            Ok(forecast) => {
                println!("天气预报更新时间: {:?}", forecast.properties.update_time);

                for period in forecast.properties.periods {
                    println!("=====================");
                    println!("时段: {}", period.name);
                    println!("温度: {} {}", period.temperature, period.temperature_unit);
                    println!(
                        "风向: {} 风速: {}",
                        period.wind_direction, period.wind_speed
                    );
                    println!("简要预报: {}", period.short_forecast);
                    println!("详细预报: {}", period.detailed_forecast);
                    println!("开始时间: {}", period.start_time);
                    println!("结束时间: {}", period.end_time);
                }
            }
            Err(e) => println!("获取天气预报失败: {:?}", e),
        }
    }
}
