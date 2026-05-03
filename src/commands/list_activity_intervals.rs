use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityInterval {
    pub id: i64,
    #[serde(rename = "type")]
    pub interval_type: String,
    pub start_index: i64,
    pub end_index: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub distance: f64,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub average_speed: f64,
    pub min_speed: f64,
    pub max_speed: f64,
    pub gap: f64,
    pub average_heartrate: Option<f64>,
    pub min_heartrate: Option<f64>,
    pub max_heartrate: Option<f64>,
    pub average_cadence: Option<f64>,
    pub min_cadence: Option<f64>,
    pub max_cadence: Option<f64>,
    pub average_watts: Option<f64>,
    pub min_watts: Option<f64>,
    pub max_watts: Option<f64>,
    pub intensity: Option<i64>,
    pub zone: Option<i64>,
    pub group_id: Option<String>,
    pub label: Option<String>,
    pub total_elevation_gain: Option<f64>,
    pub min_altitude: Option<f64>,
    pub max_altitude: Option<f64>,
    pub average_gradient: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityIntervalsResponse {
    #[serde(default)]
    pub icu_intervals: Vec<ActivityInterval>,
}

pub async fn list_activity_intervals(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<Vec<ActivityInterval>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/intervals",
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

    let wrapper = response.json::<ActivityIntervalsResponse>().await?;
    Ok(wrapper.icu_intervals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_activity_intervals_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/intervals"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "act-001",
                "analyzed": "2024-01-01T00:00:00Z",
                "icu_intervals": [
                    {
                        "id": 1,
                        "type": "WORK",
                        "start_index": 0,
                        "end_index": 300,
                        "start_time": 0,
                        "end_time": 300,
                        "distance": 1000.0,
                        "moving_time": 300,
                        "elapsed_time": 300,
                        "average_speed": 3.33,
                        "min_speed": 2.0,
                        "max_speed": 4.0,
                        "gap": 3.5,
                        "average_heartrate": 150.0,
                        "min_heartrate": 120.0,
                        "max_heartrate": 165.0,
                        "average_cadence": 85.0,
                        "min_cadence": 80,
                        "max_cadence": 90,
                        "average_watts": 250.0,
                        "min_watts": 200.0,
                        "max_watts": 300.0,
                        "intensity": 80,
                        "zone": 3,
                        "group_id": "group1",
                        "label": "Interval 1",
                        "total_elevation_gain": 5.0,
                        "min_altitude": 10.0,
                        "max_altitude": 15.0,
                        "average_gradient": 0.01
                    }
                ],
                "icu_groups": []
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let intervals = list_activity_intervals(&client, "act-001").await.unwrap();

        assert_eq!(intervals.len(), 1);
        assert_eq!(intervals[0].id, 1);
        assert_eq!(intervals[0].interval_type, "WORK");
        assert_eq!(intervals[0].distance, 1000.0);
    }

    #[tokio::test]
    async fn test_list_activity_intervals_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/intervals"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "act-001",
                "icu_intervals": [],
                "icu_groups": []
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let intervals = list_activity_intervals(&client, "act-001").await.unwrap();

        assert!(intervals.is_empty());
    }

    #[tokio::test]
    async fn test_list_activity_intervals_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/intervals"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_activity_intervals(&client, "act-001").await;

        assert!(result.is_err());
    }
}
