use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWorkoutInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub workout_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indoor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moving_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joules: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DuplicateWorkoutsInput {
    pub workout_ids: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_folder_id: Option<i32>,
}

pub async fn create_workout(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &CreateWorkoutInput,
) -> Result<super::list_workouts::Workout, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/workouts",
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

    let workout = response.json::<super::list_workouts::Workout>().await?;
    Ok(workout)
}

pub async fn update_workout(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    workout_id: i32,
    input: &CreateWorkoutInput,
) -> Result<super::list_workouts::Workout, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/workouts/{}",
        client.base_url(),
        athlete_id,
        workout_id
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

    let workout = response.json::<super::list_workouts::Workout>().await?;
    Ok(workout)
}

pub async fn delete_workout(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    workout_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/workouts/{}",
        client.base_url(),
        athlete_id,
        workout_id
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

pub async fn create_workouts_bulk(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    inputs: &[CreateWorkoutInput],
) -> Result<Vec<super::list_workouts::Workout>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/workouts/bulk",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(inputs)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let workouts = response
        .json::<Vec<super::list_workouts::Workout>>()
        .await?;
    Ok(workouts)
}

pub async fn duplicate_workouts(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &DuplicateWorkoutsInput,
) -> Result<Vec<super::list_workouts::Workout>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/duplicate-workouts",
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

    let workouts = response
        .json::<Vec<super::list_workouts::Workout>>()
        .await?;
    Ok(workouts)
}

pub async fn download_workouts_zip(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    output_path: &str,
    oldest: &str,
    newest: &str,
    ext: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/workouts.zip?oldest={}&newest={}&ext={}",
        client.base_url(),
        athlete_id,
        urlencoding::encode(oldest),
        urlencoding::encode(newest),
        urlencoding::encode(ext)
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

    let bytes = response.bytes().await?;
    std::fs::write(output_path, &bytes)?;
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
    async fn test_create_workout_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/workouts"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "name": "New Workout",
                "type": "Ride"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateWorkoutInput {
            name: Some("New Workout".to_string()),
            description: None,
            workout_type: Some("Ride".to_string()),
            indoor: None,
            moving_time: None,
            joules: None,
            folder_id: None,
        };
        let result = create_workout(&client, "a-001", &input).await.unwrap();

        assert_eq!(result.name, Some("New Workout".to_string()));
    }

    #[tokio::test]
    async fn test_update_workout_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/workouts/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "name": "Updated Workout"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateWorkoutInput {
            name: Some("Updated Workout".to_string()),
            description: None,
            workout_type: None,
            indoor: None,
            moving_time: None,
            joules: None,
            folder_id: None,
        };
        let result = update_workout(&client, "a-001", 1, &input).await.unwrap();

        assert_eq!(result.name, Some("Updated Workout".to_string()));
    }

    #[tokio::test]
    async fn test_delete_workout_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/athlete/a-001/workouts/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_workout(&client, "a-001", 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_workouts_bulk_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/workouts/bulk"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 1, "name": "Workout 1"},
                {"id": 2, "name": "Workout 2"}
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let inputs = vec![
            CreateWorkoutInput {
                name: Some("Workout 1".to_string()),
                description: None,
                workout_type: None,
                indoor: None,
                moving_time: None,
                joules: None,
                folder_id: None,
            },
            CreateWorkoutInput {
                name: Some("Workout 2".to_string()),
                description: None,
                workout_type: None,
                indoor: None,
                moving_time: None,
                joules: None,
                folder_id: None,
            },
        ];
        let result = create_workouts_bulk(&client, "a-001", &inputs)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_duplicate_workouts_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/duplicate-workouts"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 3, "name": "Copy of Workout"}
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = DuplicateWorkoutsInput {
            workout_ids: vec![1],
            target_folder_id: None,
        };
        let result = duplicate_workouts(&client, "a-001", &input).await.unwrap();

        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_download_workouts_zip_success() {
        let mock_server = MockServer::start().await;
        let output_path = std::env::temp_dir().join("test_workouts.zip");

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/workouts.zip"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"fake zip"))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = download_workouts_zip(
            &client,
            "a-001",
            output_path.to_str().unwrap(),
            "2026-01-01",
            "2026-12-31",
            ".zwo",
        )
        .await;

        assert!(result.is_ok());
        assert!(output_path.exists());
        std::fs::remove_file(&output_path).ok();
    }
}
