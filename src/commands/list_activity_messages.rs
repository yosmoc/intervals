use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityMessage {
    pub id: String,
    pub message: String,
    pub created_at: String,
    pub author: String,
}

pub async fn list_activity_messages(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<Vec<ActivityMessage>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/messages",
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

    let messages = response.json::<Vec<ActivityMessage>>().await?;
    Ok(messages)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_activity_messages_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/messages"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "msg-001",
                    "message": "Great ride!",
                    "created_at": "2024-01-15T10:00:00Z",
                    "author": "Coach"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let messages = list_activity_messages(&client, "act-001").await.unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].message, "Great ride!");
    }

    #[tokio::test]
    async fn test_list_activity_messages_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/messages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let messages = list_activity_messages(&client, "act-001").await.unwrap();

        assert!(messages.is_empty());
    }

    #[tokio::test]
    async fn test_list_activity_messages_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/messages"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_activity_messages(&client, "act-001").await;

        assert!(result.is_err());
    }
}
