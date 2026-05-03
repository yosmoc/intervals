use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MarkedActivity {
    pub id: String,
    pub start_date_local: Option<String>,
    #[serde(rename = "type")]
    pub activity_type: Option<String>,
    pub name: Option<String>,
    pub elapsed_time: Option<i64>,
    pub moving_time: Option<i64>,
    pub distance: Option<f64>,
}

pub async fn mark_event_done(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    event_id: i64,
) -> Result<MarkedActivity, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/events/{}/mark-done",
        client.base_url(),
        athlete_id,
        event_id
    );
    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let activity = response.json::<MarkedActivity>().await?;
    Ok(activity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_mark_event_done_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/events/123/mark-done"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "act-001",
                "start_date_local": "2024-01-15T08:00:00",
                "type": "Ride",
                "name": "Tempo Ride",
                "elapsed_time": 3600,
                "moving_time": 3500,
                "distance": 40000.0
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let activity = mark_event_done(&client, "a-001", 123).await.unwrap();

        assert_eq!(activity.id, "act-001");
        assert_eq!(activity.name.as_deref(), Some("Tempo Ride"));
    }

    #[tokio::test]
    async fn test_mark_event_done_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/events/123/mark-done"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = mark_event_done(&client, "a-001", 123).await;

        assert!(result.is_err());
    }
}
