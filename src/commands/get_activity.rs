use serde::Deserialize;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct Activity {
    pub id: String,
    #[serde(default)]
    pub start_date_local: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub activity_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub elapsed_time: Option<i64>,
    #[serde(default)]
    pub distance: Option<f64>,
}

pub async fn get_activity(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    activity_id: &str,
) -> Result<Activity, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/activities/{}",
        client.base_url(),
        athlete_id,
        activity_id
    );
    let response = client
        .client()
        .get(&url)
        .header("Authorization", format!("Bearer {}", client.api_key()))
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
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_activity_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/activities/act-001"))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "act-001",
                "start_date_local": "2024-01-15T08:00:00",
                "type": "Ride",
                "name": "Morning Ride",
                "elapsed_time": 3600,
                "distance": 25000.0
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let activity = get_activity(&client, "12345", "act-001").await.unwrap();

        assert_eq!(activity.id, "act-001");
        assert_eq!(activity.activity_type, Some("Ride".to_string()));
        assert_eq!(activity.name, Some("Morning Ride".to_string()));
        assert_eq!(activity.elapsed_time, Some(3600));
        assert_eq!(activity.distance, Some(25000.0));
    }

    #[tokio::test]
    async fn test_get_activity_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/activities/nonexistent"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({
                        "error": "Activity not found"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity(&client, "12345", "nonexistent").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_activity_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/activities/act-001"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_activity(&client, "12345", "act-001").await;

        assert!(result.is_err());
    }
}
