use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IcuSegment {
    pub id: i64,
    pub name: Option<String>,
    pub start_index: i64,
    pub end_index: i64,
    pub distance: Option<f64>,
    pub elapsed_time: Option<i64>,
    pub moving_time: Option<i64>,
    pub average_watts: Option<f64>,
    pub average_heartrate: Option<f64>,
    pub average_speed: Option<f64>,
}

pub async fn get_activity_segments(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<Vec<IcuSegment>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/segments",
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

    let segments = response.json::<Vec<IcuSegment>>().await?;
    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_activity_segments_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/segments"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 1,
                    "name": "Hill Climb",
                    "start_index": 100,
                    "end_index": 200,
                    "distance": 1000.0,
                    "elapsed_time": 300,
                    "moving_time": 290,
                    "average_watts": 250.0,
                    "average_heartrate": 155.0,
                    "average_speed": 3.33
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let segments = get_activity_segments(&client, "act-001").await.unwrap();

        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].name.as_deref(), Some("Hill Climb"));
    }

    #[tokio::test]
    async fn test_get_activity_segments_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/segments"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_activity_segments(&client, "act-001").await;

        assert!(result.is_err());
    }
}
