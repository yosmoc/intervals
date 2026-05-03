use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityWeatherSummary {
    pub start_index: i64,
    pub end_index: i64,
    pub start_secs: i64,
    pub end_secs: i64,
    pub moving_time: i64,
    pub whole_activity: bool,
    pub average_temp: Option<f64>,
    pub min_temp: Option<f64>,
    pub max_temp: Option<f64>,
    pub average_weather_temp: Option<f64>,
    pub min_weather_temp: Option<f64>,
    pub max_weather_temp: Option<f64>,
    pub average_feels_like: Option<f64>,
    pub min_feels_like: Option<f64>,
    pub max_feels_like: Option<f64>,
    pub wind_speed: Option<f64>,
    pub wind_gust: Option<f64>,
    pub humidity: Option<f64>,
    pub precipitation: Option<f64>,
}

pub async fn get_activity_weather_summary(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<ActivityWeatherSummary, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/weather-summary",
        client.base_url(),
        activity_id
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

    let summary = response.json::<ActivityWeatherSummary>().await?;
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_activity_weather_summary_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/weather-summary"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "start_index": 0,
                "end_index": 100,
                "start_secs": 0,
                "end_secs": 100,
                "moving_time": 100,
                "whole_activity": false,
                "temp_avg": 15.5,
                "temp_min": 10.0,
                "temp_max": 20.0,
                "feels_like_avg": 14.0,
                "wind_speed_avg": 5.0,
                "wind_gust_avg": 10.0,
                "humidity_avg": 60.0,
                "precipitation": 0.0
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let summary = get_activity_weather_summary(&client, "act-001")
            .await
            .unwrap();

        assert_eq!(summary.start_index, 0);
        assert_eq!(summary.moving_time, 100);
    }

    #[tokio::test]
    async fn test_get_activity_weather_summary_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/weather-summary"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_activity_weather_summary(&client, "act-001").await;

        assert!(result.is_err());
    }
}
