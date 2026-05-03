use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherConfig {
    pub forecasts: Option<Vec<serde_json::Value>>,
}

pub async fn get_weather_config(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<WeatherConfig, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/weather-config",
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

    let config = response.json::<WeatherConfig>().await?;
    Ok(config)
}

pub async fn update_weather_config(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    config: &WeatherConfig,
) -> Result<WeatherConfig, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/weather-config",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(config)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let config = response.json::<WeatherConfig>().await?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_weather_config_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/weather-config"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "forecasts": []
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let config = get_weather_config(&client, "a-001").await.unwrap();

        assert!(config.forecasts.is_some());
    }

    #[tokio::test]
    async fn test_update_weather_config_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/weather-config"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "forecasts": []
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let config = WeatherConfig {
            forecasts: Some(vec![]),
        };
        let result = update_weather_config(&client, "a-001", &config).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_weather_config_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/weather-config"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_weather_config(&client, "a-001").await;

        assert!(result.is_err());
    }
}
