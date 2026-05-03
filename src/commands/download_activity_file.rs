use std::path::Path;

pub async fn download_activity_file(
    client: &crate::client::ApiClient,
    activity_id: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/activity/{}/file", client.base_url(), activity_id);
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
    std::fs::write(Path::new(output_path), &bytes)?;
    Ok(())
}

pub async fn download_activity_fit_file(
    client: &crate::client::ApiClient,
    activity_id: &str,
    output_path: &str,
    power: bool,
    hr: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/activity/{}/fit-file",
        client.base_url(),
        activity_id
    );
    url.push_str(&format!("?power={}&hr={}", power, hr));

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
    std::fs::write(Path::new(output_path), &bytes)?;
    Ok(())
}

pub async fn download_activity_gpx_file(
    client: &crate::client::ApiClient,
    activity_id: &str,
    output_path: &str,
    power: bool,
    hr: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/activity/{}/gpx-file",
        client.base_url(),
        activity_id
    );
    url.push_str(&format!("?power={}&hr={}", power, hr));

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
    std::fs::write(Path::new(output_path), &bytes)?;
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
    async fn test_download_activity_file_success() {
        let mock_server = MockServer::start().await;
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_activity_file.fit");

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/file"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"fake-fit-data"))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result =
            download_activity_file(&client, "act-001", output_path.to_str().unwrap()).await;

        assert!(result.is_ok());
        assert!(output_path.exists());
        std::fs::remove_file(output_path).ok();
    }

    #[tokio::test]
    async fn test_download_activity_file_unauthorized() {
        let mock_server = MockServer::start().await;
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_activity_file_unauthorized.fit");

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/file"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result =
            download_activity_file(&client, "act-001", output_path.to_str().unwrap()).await;

        assert!(result.is_err());
    }
}
