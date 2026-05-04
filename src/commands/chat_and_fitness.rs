use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FitnessEvent {
    pub id: Option<i64>,
    pub start_date_local: Option<String>,
    pub category: Option<String>,
    pub name: Option<String>,
}

pub async fn list_fitness_model_events(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<FitnessEvent>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/fitness-model-events",
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

    let events = response.json::<Vec<FitnessEvent>>().await?;
    Ok(events)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatDetail {
    pub id: Option<i64>,
    #[serde(rename = "type")]
    pub chat_type: Option<String>,
    pub name: Option<String>,
    pub updated: Option<String>,
    pub description: Option<String>,
    pub new_message_count: Option<i64>,
    pub role: Option<String>,
    pub members: Option<Vec<serde_json::Value>>,
}

pub async fn get_chat(
    client: &crate::client::ApiClient,
    chat_id: i64,
) -> Result<ChatDetail, Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/chats/{}", client.base_url(), chat_id);
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

    let chat = response.json::<ChatDetail>().await?;
    Ok(chat)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    pub id: Option<i64>,
    pub athlete_id: Option<String>,
    pub name: Option<String>,
    pub created: Option<String>,
    #[serde(rename = "type")]
    pub message_type: Option<String>,
    pub content: Option<String>,
}

pub struct ListChatMessagesParams {
    pub chat_id: i64,
    pub before_id: Option<i64>,
    pub limit: Option<i32>,
}

pub async fn list_chat_messages(
    client: &crate::client::ApiClient,
    params: &ListChatMessagesParams,
) -> Result<Vec<ChatMessage>, Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/chats/{}/messages",
        client.base_url(),
        params.chat_id
    );
    let mut has_query = false;
    if let Some(before_id) = params.before_id {
        url.push_str(&format!("?beforeId={}", before_id));
        has_query = true;
    }
    if let Some(limit) = params.limit {
        if has_query {
            url.push('&');
        } else {
            url.push('?');
        }
        url.push_str(&format!("limit={}", limit));
    }

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

    let messages = response.json::<Vec<ChatMessage>>().await?;
    Ok(messages)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SendResponse {
    pub message_id: Option<i64>,
    pub chat_id: Option<i64>,
}

pub async fn send_chat_message(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    content: &str,
    chat_id: Option<i64>,
) -> Result<SendResponse, Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/chats/send-message", client.base_url());
    let mut body = serde_json::json!({
        "athlete_id": athlete_id,
        "content": content
    });
    if let Some(cid) = chat_id {
        body["chat_id"] = serde_json::json!(cid);
    }

    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body_text = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body_text).into());
    }

    let result = response.json::<SendResponse>().await?;
    Ok(result)
}

pub async fn update_chat_message(
    client: &crate::client::ApiClient,
    chat_id: i64,
    msg_id: i64,
    content: &str,
) -> Result<ChatMessage, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/chats/{}/messages/{}",
        client.base_url(),
        chat_id,
        msg_id
    );
    let body = serde_json::json!({ "content": content });
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body_text = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body_text).into());
    }

    let result = response.json::<ChatMessage>().await?;
    Ok(result)
}

pub async fn delete_chat_message(
    client: &crate::client::ApiClient,
    chat_id: i64,
    msg_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/chats/{}/messages/{}",
        client.base_url(),
        chat_id,
        msg_id
    );
    let response = client
        .client()
        .delete(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body_text = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body_text).into());
    }

    Ok(())
}

pub async fn mark_chat_messages_seen(
    client: &crate::client::ApiClient,
    chat_id: i64,
    msg_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/chats/{}/messages/{}/seen",
        client.base_url(),
        chat_id,
        msg_id
    );
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body_text = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body_text).into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_fitness_model_events_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/fitness-model-events"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 1,
                    "start_date_local": "2024-01-01T00:00:00",
                    "category": "SET_FITNESS",
                    "name": "Starting fitness"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let events = list_fitness_model_events(&client, "a-001").await.unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].category.as_deref(), Some("SET_FITNESS"));
    }

    #[tokio::test]
    async fn test_get_chat_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/chats/123"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 123,
                "type": "PRIVATE",
                "name": "Coach Chat"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let chat = get_chat(&client, 123).await.unwrap();

        assert_eq!(chat.id, Some(123));
        assert_eq!(chat.name.as_deref(), Some("Coach Chat"));
    }

    #[tokio::test]
    async fn test_list_chat_messages_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/chats/123/messages"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 1,
                    "athlete_id": "a-001",
                    "name": "Coach",
                    "created": "2024-01-15T10:00:00Z",
                    "type": "TEXT",
                    "content": "Great work!"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let params = ListChatMessagesParams {
            chat_id: 123,
            before_id: None,
            limit: Some(10),
        };
        let messages = list_chat_messages(&client, &params).await.unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content.as_deref(), Some("Great work!"));
    }

    #[tokio::test]
    async fn test_list_fitness_model_events_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/fitness-model-events"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_fitness_model_events(&client, "a-001").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_chat_message_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/chats/send-message"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "message_id": 123,
                "chat_id": 456
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = send_chat_message(&client, "a-001", "Hello!", Some(456))
            .await
            .unwrap();

        assert_eq!(result.message_id, Some(123));
    }

    #[tokio::test]
    async fn test_update_chat_message_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/chats/123/messages/456"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 456,
                "content": "Updated message"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = update_chat_message(&client, 123, 456, "Updated message")
            .await
            .unwrap();

        assert_eq!(result.content.as_deref(), Some("Updated message"));
    }

    #[tokio::test]
    async fn test_delete_chat_message_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/chats/123/messages/456"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_chat_message(&client, 123, 456).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mark_chat_messages_seen_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/chats/123/messages/456/seen"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = mark_chat_messages_seen(&client, 123, 456).await;

        assert!(result.is_ok());
    }
}
