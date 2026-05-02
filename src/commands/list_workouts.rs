use serde::Deserialize;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct Workout {
    pub id: i32,
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
    pub moving_time: Option<i32>,
    #[serde(default)]
    pub joules: Option<i32>,
    #[serde(default)]
    pub folder_id: Option<i32>,
}

pub async fn list_workouts(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<Workout>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/workouts",
        client.base_url(),
        athlete_id
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

    let workouts = response.json::<Vec<Workout>>().await?;
    Ok(workouts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_workouts_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/workouts"))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 1,
                    "name": "Threshold Intervals",
                    "description": "6x5min at threshold",
                    "type": "Ride",
                    "indoor": false,
                    "moving_time": 3600,
                    "joules": 1500000,
                    "folder_id": 10
                },
                {
                    "id": 2,
                    "name": "Easy Recovery",
                    "description": "Zone 2 spin",
                    "type": "Ride",
                    "indoor": true,
                    "moving_time": 1800,
                    "joules": 800000,
                    "folder_id": 10
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let workouts = list_workouts(&client, "12345").await.unwrap();

        assert_eq!(workouts.len(), 2);
        assert_eq!(workouts[0].id, 1);
        assert_eq!(workouts[0].name, Some("Threshold Intervals".to_string()));
        assert_eq!(workouts[1].id, 2);
    }

    #[tokio::test]
    async fn test_list_workouts_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/workouts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let workouts = list_workouts(&client, "12345").await.unwrap();

        assert!(workouts.is_empty());
    }

    #[tokio::test]
    async fn test_list_workouts_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/workouts"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_workouts(&client, "12345").await;

        assert!(result.is_err());
    }
}
