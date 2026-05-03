use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateActivityInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub activity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sport: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Activity {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub activity_type: Option<String>,
    #[serde(default)]
    pub sport: Option<String>,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub elapsed_time: Option<i64>,
}

pub async fn update_activity(
    client: &crate::client::ApiClient,
    activity_id: &str,
    input: &UpdateActivityInput,
) -> Result<Activity, Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/activity/{}", client.base_url(), activity_id);

    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(input)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let activity = response.json::<Activity>().await?;
    Ok(activity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{body_partial_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_update_activity_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/activity/act-001"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .and(body_partial_json(serde_json::json!({
                "name": "Updated Ride"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "act-001",
                "name": "Updated Ride",
                "description": "My updated ride",
                "type": "Ride",
                "sport": "Cycling",
                "distance": 30000.0,
                "elapsed_time": 4200
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = UpdateActivityInput {
            name: Some("Updated Ride".to_string()),
            description: Some("My updated ride".to_string()),
            activity_type: None,
            sport: None,
            distance: None,
            elapsed_time: None,
        };
        let activity = update_activity(&client, "act-001", &input).await.unwrap();

        assert_eq!(activity.id, "act-001");
        assert_eq!(activity.name, Some("Updated Ride".to_string()));
    }

    #[tokio::test]
    async fn test_update_activity_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/activity/nonexistent"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "error": "Activity not found"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = UpdateActivityInput {
            name: Some("Test".to_string()),
            description: None,
            activity_type: None,
            sport: None,
            distance: None,
            elapsed_time: None,
        };
        let result = update_activity(&client, "nonexistent", &input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_activity_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/activity/act-001"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let input = UpdateActivityInput {
            name: Some("Test".to_string()),
            description: None,
            activity_type: None,
            sport: None,
            distance: None,
            elapsed_time: None,
        };
        let result = update_activity(&client, "act-001", &input).await;

        assert!(result.is_err());
    }
}
