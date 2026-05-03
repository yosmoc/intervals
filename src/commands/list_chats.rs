use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Chat {
    pub id: String,
    pub name: String,
    pub last_message: String,
    pub updated_at: String,
}

pub async fn list_chats(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<Chat>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/chats",
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

    let chats = response.json::<Vec<Chat>>().await?;
    Ok(chats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_chats_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/chats"))
            .and(header("Authorization", "Basic QVBJX0tFWTp0ZXN0LWFwaS1rZXk="))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "chat-001",
                    "name": "Coach Chat",
                    "last_message": "Looks good!",
                    "updated_at": "2024-01-15T10:00:00Z"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let chats = list_chats(&client, "12345").await.unwrap();

        assert_eq!(chats.len(), 1);
        assert_eq!(chats[0].name, "Coach Chat");
    }

    #[tokio::test]
    async fn test_list_chats_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/chats"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let chats = list_chats(&client, "12345").await.unwrap();

        assert!(chats.is_empty());
    }

    #[tokio::test]
    async fn test_list_chats_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/chats"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_chats(&client, "12345").await;

        assert!(result.is_err());
    }
}
