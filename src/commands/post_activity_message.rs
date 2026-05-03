use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostActivityMessageResponse {
    pub id: i64,
}

pub async fn post_activity_message(
    client: &crate::client::ApiClient,
    activity_id: &str,
    content: &str,
) -> Result<PostActivityMessageResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/messages",
        client.base_url(),
        activity_id
    );
    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(&serde_json::json!({ "content": content }))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let result = response.json::<PostActivityMessageResponse>().await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_post_activity_message_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/activity/act-001/messages"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .and(body_json(
                serde_json::json!({ "content": "Great workout!" }),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 12345
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = post_activity_message(&client, "act-001", "Great workout!")
            .await
            .unwrap();

        assert_eq!(result.id, 12345);
    }

    #[tokio::test]
    async fn test_post_activity_message_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/activity/act-001/messages"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = post_activity_message(&client, "act-001", "Test").await;

        assert!(result.is_err());
    }
}
