use serde::{Deserialize, Serialize};

pub async fn disconnect_app(
    client: &crate::client::ApiClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/disconnect-app", client.base_url());
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

#[derive(Debug, Deserialize, Serialize)]
pub struct SharedEvent {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub start_date: Option<String>,
    pub description: Option<String>,
    pub courses: Option<serde_json::Value>,
}

pub async fn get_shared_event(
    client: &crate::client::ApiClient,
    event_id: i64,
) -> Result<SharedEvent, Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/shared-event/{}", client.base_url(), event_id);
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

    let event = response.json::<SharedEvent>().await?;
    Ok(event)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaceDistances {
    pub distances: Option<Vec<serde_json::Value>>,
}

pub async fn list_pace_distances(
    client: &crate::client::ApiClient,
) -> Result<PaceDistances, Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/pace_distances", client.base_url());
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

    let distances = response.json::<PaceDistances>().await?;
    Ok(distances)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_disconnect_app_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/disconnect-app"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = disconnect_app(&client).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_shared_event_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/shared-event/123"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 123,
                "name": "Local Race",
                "start_date": "2024-06-15"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let event = get_shared_event(&client, 123).await.unwrap();

        assert_eq!(event.id, Some(123));
        assert_eq!(event.name.as_deref(), Some("Local Race"));
    }

    #[tokio::test]
    async fn test_list_pace_distances_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/pace_distances"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "distances": [400, 800, 1600, 5000]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let distances = list_pace_distances(&client).await.unwrap();

        assert!(distances.distances.is_some());
    }

    #[tokio::test]
    async fn test_disconnect_app_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/disconnect-app"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = disconnect_app(&client).await;

        assert!(result.is_err());
    }
}
