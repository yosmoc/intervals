use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IntervalStats {
    pub start_index: Option<i64>,
    pub distance: Option<f64>,
    pub moving_time: Option<i64>,
    pub elapsed_time: Option<i64>,
    pub average_watts: Option<i64>,
    pub average_watts_alt: Option<i64>,
    pub min_watts: Option<i64>,
    pub max_watts: Option<i64>,
    pub average_watts_kg: Option<f64>,
    pub max_watts_kg: Option<f64>,
    pub intensity: Option<i64>,
    pub weighted_average_watts: Option<i64>,
    pub training_load: Option<f64>,
    pub joules: Option<i64>,
    pub average_heartrate: Option<f64>,
    pub max_heartrate: Option<i64>,
    pub average_speed: Option<f64>,
    pub average_cadence: Option<f64>,
    pub average_temp: Option<f64>,
    pub total_elevation_gain: Option<f64>,
}

pub async fn get_interval_stats(
    client: &crate::client::ApiClient,
    activity_id: &str,
    start_index: i64,
    end_index: i64,
) -> Result<IntervalStats, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/interval-stats?start_index={}&end_index={}",
        client.base_url(),
        activity_id,
        start_index,
        end_index
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

    let stats = response.json::<IntervalStats>().await?;
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_interval_stats_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/interval-stats"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "start_index": 100,
                "distance": 1000.0,
                "moving_time": 300,
                "average_watts": 250,
                "weighted_average_watts": 260,
                "training_load": 50.0,
                "average_heartrate": 155.0
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let stats = get_interval_stats(&client, "act-001", 100, 200)
            .await
            .unwrap();

        assert_eq!(stats.start_index, Some(100));
        assert_eq!(stats.average_watts, Some(250));
    }

    #[tokio::test]
    async fn test_get_interval_stats_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/interval-stats"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_interval_stats(&client, "act-001", 100, 200).await;

        assert!(result.is_err());
    }
}
