use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadedActivity {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub activity_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ManualActivityInput {
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
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkManualActivitiesInput {
    pub activities: Vec<ManualActivityInput>,
}

pub async fn upload_activity(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    file_path: &str,
) -> Result<UploadedActivity, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/activities",
        client.base_url(),
        athlete_id
    );

    let file_bytes = std::fs::read(file_path)?;
    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("activity.fit");

    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .header("Content-Type", "application/octet-stream")
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", file_name),
        )
        .body(file_bytes)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let result = response.json::<UploadedActivity>().await?;
    Ok(result)
}

pub async fn create_manual_activities_bulk(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &BulkManualActivitiesInput,
) -> Result<Vec<UploadedActivity>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/activities/manual/bulk",
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

    let result = response.json::<Vec<UploadedActivity>>().await?;
    Ok(result)
}

pub async fn download_activity_fit_files(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/download-fit-files",
        client.base_url(),
        athlete_id
    );

    let response = client
        .client()
        .post(&url)
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
    async fn test_upload_activity_success() {
        let mock_server = MockServer::start().await;

        let temp_file = std::env::temp_dir().join("test_activity.fit");
        std::fs::write(&temp_file, b"fake fit data").unwrap();

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/12345/activities"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "act-001",
                "name": "Uploaded Activity",
                "activity_type": "Ride"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = upload_activity(&client, "12345", temp_file.to_str().unwrap())
            .await
            .unwrap();

        assert_eq!(result.id, "act-001");

        std::fs::remove_file(&temp_file).ok();
    }

    #[tokio::test]
    async fn test_upload_activity_unauthorized() {
        let mock_server = MockServer::start().await;

        let temp_file = std::env::temp_dir().join("test_activity_unauth.fit");
        std::fs::write(&temp_file, b"fake fit data").unwrap();

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/12345/activities"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = upload_activity(&client, "12345", temp_file.to_str().unwrap()).await;

        assert!(result.is_err());

        std::fs::remove_file(&temp_file).ok();
    }

    #[tokio::test]
    async fn test_create_manual_activities_bulk_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/12345/activities/manual/bulk"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": "act-001", "name": "Manual Run"},
                {"id": "act-002", "name": "Manual Ride"}
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = BulkManualActivitiesInput {
            activities: vec![
                ManualActivityInput {
                    name: Some("Manual Run".to_string()),
                    description: None,
                    activity_type: Some("Run".to_string()),
                    sport: None,
                    start_date: None,
                    elapsed_time: None,
                    distance: None,
                },
                ManualActivityInput {
                    name: Some("Manual Ride".to_string()),
                    description: None,
                    activity_type: Some("Ride".to_string()),
                    sport: None,
                    start_date: None,
                    elapsed_time: None,
                    distance: None,
                },
            ],
        };
        let result = create_manual_activities_bulk(&client, "12345", &input)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, Some("Manual Run".to_string()));
    }

    #[tokio::test]
    async fn test_download_activity_fit_files_success() {
        let mock_server = MockServer::start().await;

        let output_file = std::env::temp_dir().join("test_fit_files.zip");

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/12345/download-fit-files"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"fake zip data"))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result =
            download_activity_fit_files(&client, "12345", output_file.to_str().unwrap()).await;

        assert!(result.is_ok());
        assert!(output_file.exists());

        std::fs::remove_file(&output_file).ok();
    }
}
