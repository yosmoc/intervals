use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherForecast {
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub temp_min: Option<f64>,
    #[serde(default)]
    pub temp_max: Option<f64>,
    #[serde(default)]
    pub condition: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherForecastResponse {
    #[serde(default)]
    pub forecasts: Vec<WeatherForecast>,
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

    let wrapper = response.json::<WeatherForecastResponse>().await?;
    Ok(wrapper.forecasts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_weather_forecast_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/weather-forecast"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "forecasts": [
                    {
                        "date": "2024-01-15",
                        "temp_min": 2.0,
                        "temp_max": 8.0,
                        "condition": "Cloudy",
                        "icon": "cloudy"
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let forecasts = get_weather_forecast(&client, "12345").await.unwrap();

        assert_eq!(forecasts.len(), 1);
        assert_eq!(forecasts[0].temp_max, Some(8.0));
    }

    #[tokio::test]
    async fn test_get_weather_forecast_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/weather-forecast"))
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
