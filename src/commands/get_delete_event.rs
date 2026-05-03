use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: i64,
    pub start_date_local: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub category: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub uid: Option<String>,
    pub notes: Option<String>,
    pub workout: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventEx {
    pub id: Option<i64>,
    pub start_date_local: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub category: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub uid: Option<String>,
    pub notes: Option<String>,
    pub workout: Option<serde_json::Value>,
}

pub async fn get_event(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    event_id: i64,
) -> Result<Event, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/events/{}",
        client.base_url(),
        athlete_id,
        event_id
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

    let event = response.json::<Event>().await?;
    Ok(event)
}

pub async fn delete_event(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    event_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/events/{}",
        client.base_url(),
        athlete_id,
        event_id
    );
    let response = client
        .client()
        .delete(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    Ok(())
}

pub async fn update_event(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    event_id: i64,
    event: &EventEx,
) -> Result<Event, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/events/{}",
        client.base_url(),
        athlete_id,
        event_id
    );
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(event)
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

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_event_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/events/123"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 123,
                "start_date_local": "2024-01-15T08:00:00",
                "type": "WORKOUT",
                "category": "WORKOUT",
                "name": "Tempo Ride",
                "description": null,
                "uid": "uid-001"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let event = get_event(&client, "a-001", 123).await.unwrap();

        assert_eq!(event.id, 123);
        assert_eq!(event.name.as_deref(), Some("Tempo Ride"));
    }

    #[tokio::test]
    async fn test_delete_event_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/athlete/a-001/events/123"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_event(&client, "a-001", 123).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_event_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/events/123"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_event(&client, "a-001", 123).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_event_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/events/123"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 123,
                "start_date_local": "2024-01-15T08:00:00",
                "type": "WORKOUT",
                "category": "WORKOUT",
                "name": "Updated Tempo Ride",
                "description": "Updated description",
                "uid": "uid-001"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let update = EventEx {
            id: Some(123),
            start_date_local: Some("2024-01-15T08:00:00".to_string()),
            event_type: Some("WORKOUT".to_string()),
            category: Some("WORKOUT".to_string()),
            name: Some("Updated Tempo Ride".to_string()),
            description: Some("Updated description".to_string()),
            uid: Some("uid-001".to_string()),
            notes: None,
            workout: None,
        };
        let event = update_event(&client, "a-001", 123, &update).await.unwrap();

        assert_eq!(event.id, 123);
        assert_eq!(event.name.as_deref(), Some("Updated Tempo Ride"));
    }
}
