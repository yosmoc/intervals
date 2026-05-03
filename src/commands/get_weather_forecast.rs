use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherForecast {
    pub date: String,
    pub temp_min: f64,
    pub temp_max: f64,
    pub condition: String,
    pub icon: String,
}

pub async fn get_weather_forecast(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<WeatherForecast>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/weather-forecast",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .get(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let forecast = response.json::<Vec<WeatherForecast>>().await?;
    Ok(forecast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_weather_forecast_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/weather-forecast"))
            .and(header("Authorization", "Basic QVBJX0tFWTp0ZXN0LWFwaS1rZXk="))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "date": "2024-01-15",
                    "temp_min": 5.0,
                    "temp_max": 12.0,
                    "condition": "Partly Cloudy",
                    "icon": "partly-cloudy"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let forecast = get_weather_forecast(&client, "12345").await.unwrap();

        assert_eq!(forecast.len(), 1);
        assert_eq!(forecast[0].condition, "Partly Cloudy");
    }

    #[tokio::test]
    async fn test_get_weather_forecast_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/weather-forecast"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_weather_forecast(&client, "12345").await;

        assert!(result.is_err());
    }
}
