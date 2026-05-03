use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Folder {
    pub id: i32,
    #[serde(default)]
    pub athlete_id: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub folder_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub start_date_local: Option<String>,
    #[serde(default)]
    pub children: Option<Vec<FolderWorkout>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FolderWorkout {
    pub id: i32,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub workout_type: Option<String>,
    #[serde(default)]
    pub moving_time: Option<i32>,
    #[serde(default)]
    pub joules: Option<i32>,
}

pub async fn list_folders(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<Folder>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/folders",
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

    let folders = response.json::<Vec<Folder>>().await?;
    Ok(folders)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_folders_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/folders"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 1,
                    "type": "FOLDER",
                    "name": "Base Training",
                    "description": "Base phase workouts",
                    "visibility": "PRIVATE",
                    "children": [
                        {
                            "id": 10,
                            "name": "Easy Ride",
                            "type": "Ride",
                            "moving_time": 3600,
                            "joules": 1200000
                        }
                    ]
                },
                {
                    "id": 2,
                    "type": "PLAN",
                    "name": "12 Week Plan",
                    "description": "Full season plan",
                    "visibility": "PRIVATE",
                    "start_date_local": "2024-01-01",
                    "rollout_weeks": 12
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let folders = list_folders(&client, "12345").await.unwrap();

        assert_eq!(folders.len(), 2);
        assert_eq!(folders[0].id, 1);
        assert_eq!(folders[0].folder_type, Some("FOLDER".to_string()));
        assert_eq!(folders[1].id, 2);
        assert_eq!(folders[1].folder_type, Some("PLAN".to_string()));
    }

    #[tokio::test]
    async fn test_list_folders_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/folders"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let folders = list_folders(&client, "12345").await.unwrap();

        assert!(folders.is_empty());
    }

    #[tokio::test]
    async fn test_list_folders_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/folders"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_folders(&client, "12345").await;

        assert!(result.is_err());
    }
}
