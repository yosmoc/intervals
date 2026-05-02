use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityId {
    pub id: String,
}

pub async fn delete_activity(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<ActivityId, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}",
        client.base_url(),
        activity_id
    );

    let response = client
        .client()
        .delete(&url)
        .header("Authorization", format!("Bearer {}", client.api_key()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let result = response.json::<ActivityId>().await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_delete_activity_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/activity/act-001"))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "act-001"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_activity(&client, "act-001").await.unwrap();

        assert_eq!(result.id, "act-001");
    }

    #[tokio::test]
    async fn test_delete_activity_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/activity/nonexistent"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({
                        "error": "Activity not found"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_activity(&client, "nonexistent").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_activity_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/activity/act-001"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = delete_activity(&client, "act-001").await;

        assert!(result.is_err());
    }
}
