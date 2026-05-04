use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SharedWith {
    pub athlete_id: Option<String>,
    pub athlete_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFolderInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub folder_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFolderSharedWithInput {
    pub athlete_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFolderWorkoutsInput {
    pub workout_ids: Vec<i32>,
}

pub async fn create_folder(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &CreateFolderInput,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/folders",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(input)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let result = response.json::<serde_json::Value>().await?;
    Ok(result)
}

pub async fn update_folder(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    folder_id: i64,
    input: &CreateFolderInput,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/folders/{}",
        client.base_url(),
        athlete_id,
        folder_id
    );
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

    let result = response.json::<serde_json::Value>().await?;
    Ok(result)
}

pub async fn update_folder_shared_with(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    folder_id: i64,
    input: &UpdateFolderSharedWithInput,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/folders/{}/shared-with",
        client.base_url(),
        athlete_id,
        folder_id
    );
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

    Ok(())
}

pub async fn update_folder_workouts(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    folder_id: i64,
    input: &UpdateFolderWorkoutsInput,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/folders/{}/workouts",
        client.base_url(),
        athlete_id,
        folder_id
    );
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

    Ok(())
}

pub async fn list_folder_shared_with(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    folder_id: i64,
) -> Result<Vec<SharedWith>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/folders/{}/shared-with",
        client.base_url(),
        athlete_id,
        folder_id
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

    let shared = response.json::<Vec<SharedWith>>().await?;
    Ok(shared)
}

pub async fn delete_folder(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    folder_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/folders/{}",
        client.base_url(),
        athlete_id,
        folder_id
    );
    let response = client
        .client()
        .delete(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
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
    async fn test_list_folder_shared_with_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/folders/1/shared-with"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "athlete_id": "a-002",
                    "athlete_name": "Coach"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let shared = list_folder_shared_with(&client, "a-001", 1).await.unwrap();

        assert_eq!(shared.len(), 1);
        assert_eq!(shared[0].athlete_name.as_deref(), Some("Coach"));
    }

    #[tokio::test]
    async fn test_delete_folder_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/athlete/a-001/folders/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_folder(&client, "a-001", 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_folder_shared_with_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/folders/1/shared-with"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_folder_shared_with(&client, "a-001", 1).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_folder_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/folders"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "name": "New Folder"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateFolderInput {
            name: Some("New Folder".to_string()),
            description: None,
            folder_type: None,
        };
        let result = create_folder(&client, "a-001", &input).await.unwrap();

        assert_eq!(
            result.get("name").and_then(|v| v.as_str()),
            Some("New Folder")
        );
    }

    #[tokio::test]
    async fn test_update_folder_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/folders/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "name": "Updated Folder"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateFolderInput {
            name: Some("Updated Folder".to_string()),
            description: None,
            folder_type: None,
        };
        let result = update_folder(&client, "a-001", 1, &input).await.unwrap();

        assert_eq!(
            result.get("name").and_then(|v| v.as_str()),
            Some("Updated Folder")
        );
    }

    #[tokio::test]
    async fn test_update_folder_shared_with_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/folders/1/shared-with"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = UpdateFolderSharedWithInput {
            athlete_ids: vec!["a-002".to_string()],
        };
        let result = update_folder_shared_with(&client, "a-001", 1, &input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_folder_workouts_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/folders/1/workouts"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = UpdateFolderWorkoutsInput {
            workout_ids: vec![1, 2],
        };
        let result = update_folder_workouts(&client, "a-001", 1, &input).await;

        assert!(result.is_ok());
    }
}
