use serde::{Deserialize, Serialize};

pub async fn download_activities_csv(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/activities.csv",
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

    let bytes = response.bytes().await?;
    std::fs::write(std::path::Path::new(output_path), &bytes)?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WellnessUpdate {
    pub ctl: Option<f64>,
    pub atl: Option<f64>,
    pub weight: Option<f64>,
    pub resting_hr: Option<i64>,
    pub hrv: Option<f64>,
    pub mood: Option<i64>,
    pub fatigue: Option<i64>,
    pub motivation: Option<i64>,
    pub sleep: Option<f64>,
    pub sleep_quality: Option<i64>,
    pub soreness: Option<i64>,
    pub stress: Option<i64>,
    pub fitness: Option<i64>,
    pub readiness: Option<i64>,
}

pub async fn update_wellness(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    date: &str,
    record: &WellnessUpdate,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/wellness/{}",
        client.base_url(),
        athlete_id,
        date
    );
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(record)
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkWellnessUpdate {
    pub records: Vec<WellnessUpdateWithDate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WellnessUpdateWithDate {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resting_hr: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrv: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fatigue: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_quality: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soreness: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stress: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fitness: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness: Option<i64>,
}

pub async fn update_wellness_bulk(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    records: &[WellnessUpdateWithDate],
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/wellness-bulk",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(records)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    Ok(())
}

pub async fn upload_wellness_csv(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/wellness",
        client.base_url(),
        athlete_id
    );
    let csv_content = std::fs::read_to_string(file_path)?;
    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .header("Content-Type", "text/csv")
        .body(csv_content)
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
    async fn test_download_activities_csv_success() {
        let mock_server = MockServer::start().await;
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_activities.csv");

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/activities.csv"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(
                ResponseTemplate::new(200).set_body_bytes(b"id,name,date\ni1,Run,2024-01-15"),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = download_activities_csv(&client, "a-001", output_path.to_str().unwrap()).await;

        assert!(result.is_ok());
        assert!(output_path.exists());
        std::fs::remove_file(output_path).ok();
    }

    #[tokio::test]
    async fn test_update_wellness_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/wellness/2024-01-15"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "2024-01-15",
                "weight": 75.0
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let record = WellnessUpdate {
            ctl: None,
            atl: None,
            weight: Some(75.0),
            resting_hr: None,
            hrv: None,
            mood: None,
            fatigue: None,
            motivation: None,
            sleep: None,
            sleep_quality: None,
            soreness: None,
            stress: None,
            fitness: None,
            readiness: None,
        };
        let result = update_wellness(&client, "a-001", "2024-01-15", &record).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_download_activities_csv_unauthorized() {
        let mock_server = MockServer::start().await;
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_activities_unauthorized.csv");

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/activities.csv"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = download_activities_csv(&client, "a-001", output_path.to_str().unwrap()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_wellness_bulk_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/wellness-bulk"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let records = vec![WellnessUpdateWithDate {
            id: "2024-01-15".to_string(),
            ctl: None,
            atl: None,
            weight: Some(75.0),
            resting_hr: None,
            hrv: None,
            mood: None,
            fatigue: None,
            motivation: None,
            sleep: None,
            sleep_quality: None,
            soreness: None,
            stress: None,
            fitness: None,
            readiness: None,
        }];
        let result = update_wellness_bulk(&client, "a-001", &records).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_upload_wellness_csv_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/wellness"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let temp_file = std::env::temp_dir().join("test_wellness.csv");
        std::fs::write(&temp_file, "date,weight,resting_hr\n2024-01-15,75.0,50").unwrap();

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = upload_wellness_csv(&client, "a-001", temp_file.to_str().unwrap()).await;

        assert!(result.is_ok());
        std::fs::remove_file(&temp_file).ok();
    }
}
