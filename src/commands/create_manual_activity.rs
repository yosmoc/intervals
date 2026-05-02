use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateManualActivityInput {
    pub start_date_local: String,
    #[serde(rename = "type")]
    pub activity_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
    pub start_date_local: Option<String>,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub elapsed_time: Option<i64>,
}

pub async fn create_manual_activity(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &CreateManualActivityInput,
) -> Result<Activity, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/activities/manual",
        client.base_url(),
        athlete_id
    );

    let response = client
        .client()
        .post(&url)
        .header("Authorization", format!("Bearer {}", client.api_key()))
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
    use wiremock::matchers::{body_partial_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_create_manual_activity_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/12345/activities/manual"))
            .and(header("Authorization", "Bearer test-api-key"))
            .and(body_partial_json(serde_json::json!({
                "start_date_local": "2024-01-15T08:00:00",
                "type": "Ride"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "act-new-001",
                "name": "Morning Ride",
                "description": "Manual activity",
                "type": "Ride",
                "start_date_local": "2024-01-15T08:00:00",
                "distance": 25000.0,
                "elapsed_time": 3600
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateManualActivityInput {
            start_date_local: "2024-01-15T08:00:00".to_string(),
            activity_type: "Ride".to_string(),
            name: Some("Morning Ride".to_string()),
            description: Some("Manual activity".to_string()),
            sport: None,
            distance: Some(25000.0),
            elapsed_time: Some(3600),
        };
        let activity = create_manual_activity(&client, "12345", &input).await.unwrap();

        assert_eq!(activity.id, "act-new-001");
        assert_eq!(activity.name, Some("Morning Ride".to_string()));
        assert_eq!(activity.activity_type, Some("Ride".to_string()));
    }

    #[tokio::test]
    async fn test_create_manual_activity_minimal() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/12345/activities/manual"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "act-new-002",
                "name": null,
                "description": null,
                "type": "Run",
                "start_date_local": "2024-01-16T07:00:00",
                "distance": null,
                "elapsed_time": null
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateManualActivityInput {
            start_date_local: "2024-01-16T07:00:00".to_string(),
            activity_type: "Run".to_string(),
            name: None,
            description: None,
            sport: None,
            distance: None,
            elapsed_time: None,
        };
        let activity = create_manual_activity(&client, "12345", &input).await.unwrap();

        assert_eq!(activity.id, "act-new-002");
        assert_eq!(activity.activity_type, Some("Run".to_string()));
    }

    #[tokio::test]
    async fn test_create_manual_activity_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/12345/activities/manual"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let input = CreateManualActivityInput {
            start_date_local: "2024-01-15T08:00:00".to_string(),
            activity_type: "Ride".to_string(),
            name: None,
            description: None,
            sport: None,
            distance: None,
            elapsed_time: None,
        };
        let result = create_manual_activity(&client, "12345", &input).await;

        assert!(result.is_err());
    }
}
