use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateEventInput {
    pub start_date_local: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: i32,
    #[serde(default)]
    pub start_date_local: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub uid: Option<String>,
}

pub async fn create_event(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &CreateEventInput,
    upsert_on_uid: bool,
) -> Result<Event, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/events?upsertOnUid={}",
        client.base_url(),
        athlete_id,
        upsert_on_uid
    );

    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(input)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let event = response.json::<Event>().await?;
    Ok(event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{body_partial_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_create_event_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path_regex("/api/v1/athlete/.*/events"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .and(body_partial_json(serde_json::json!({
                "start_date_local": "2024-01-15T08:00:00",
                "type": "WORKOUT",
                "category": "WORKOUT"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 201,
                "start_date_local": "2024-01-15T08:00:00",
                "type": "WORKOUT",
                "category": "WORKOUT",
                "name": "Threshold Intervals",
                "description": "6x5min at threshold",
                "uid": "evt-001"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateEventInput {
            start_date_local: "2024-01-15T08:00:00".to_string(),
            event_type: "WORKOUT".to_string(),
            category: "WORKOUT".to_string(),
            name: Some("Threshold Intervals".to_string()),
            description: Some("6x5min at threshold".to_string()),
            uid: Some("evt-001".to_string()),
            calendar_id: None,
        };
        let event = create_event(&client, "12345", &input, false).await.unwrap();

        assert_eq!(event.id, 201);
        assert_eq!(event.name, Some("Threshold Intervals".to_string()));
    }

    #[tokio::test]
    async fn test_create_event_note() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path_regex("/api/v1/athlete/.*/events"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 202,
                "start_date_local": "2024-01-16T07:00:00",
                "type": "NOTE",
                "category": "NOTE",
                "name": "Rest Day",
                "description": "Take it easy",
                "uid": null
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateEventInput {
            start_date_local: "2024-01-16T07:00:00".to_string(),
            event_type: "NOTE".to_string(),
            category: "NOTE".to_string(),
            name: Some("Rest Day".to_string()),
            description: Some("Take it easy".to_string()),
            uid: None,
            calendar_id: None,
        };
        let event = create_event(&client, "12345", &input, false).await.unwrap();

        assert_eq!(event.id, 202);
        assert_eq!(event.category, Some("NOTE".to_string()));
    }

    #[tokio::test]
    async fn test_create_event_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path_regex("/api/v1/athlete/.*/events"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let input = CreateEventInput {
            start_date_local: "2024-01-15T08:00:00".to_string(),
            event_type: "WORKOUT".to_string(),
            category: "WORKOUT".to_string(),
            name: None,
            description: None,
            uid: None,
            calendar_id: None,
        };
        let result = create_event(&client, "12345", &input, false).await;

        assert!(result.is_err());
    }
}
