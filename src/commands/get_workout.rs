use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Workout {
    pub id: i32,
    #[serde(default)]
    pub athlete_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub workout_type: Option<String>,
    #[serde(default)]
    pub indoor: Option<bool>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub moving_time: Option<i32>,
    #[serde(default)]
    pub joules: Option<i32>,
    #[serde(default)]
    pub joules_above_ftp: Option<i32>,
    #[serde(default)]
    pub folder_id: Option<i32>,
    #[serde(default)]
    pub workout_doc: Option<serde_json::Value>,
}

pub async fn get_workout(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    workout_id: i32,
) -> Result<Workout, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/workouts/{}",
        client.base_url(),
        athlete_id,
        workout_id
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

    let workout = response.json::<Workout>().await?;
    Ok(workout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_workout_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/workouts/42"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 42,
                "athlete_id": "12345",
                "name": "Threshold Intervals",
                "description": "6x5min at threshold",
                "type": "Ride",
                "indoor": false,
                "color": "#FF0000",
                "moving_time": 3600,
                "joules": 1500000,
                "joules_above_ftp": 800000,
                "folder_id": 10
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let workout = get_workout(&client, "12345", 42).await.unwrap();

        assert_eq!(workout.id, 42);
        assert_eq!(workout.name, Some("Threshold Intervals".to_string()));
        assert_eq!(workout.workout_type, Some("Ride".to_string()));
    }

    #[tokio::test]
    async fn test_get_workout_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/workouts/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "error": "Workout not found"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_workout(&client, "12345", 999).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_workout_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/workouts/42"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_workout(&client, "12345", 42).await;

        assert!(result.is_err());
    }
}
