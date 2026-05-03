pub async fn get_athlete_settings(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    device_class: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/settings/{}",
        client.base_url(),
        athlete_id,
        device_class
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

    let settings = response.json::<serde_json::Value>().await?;
    Ok(settings)
}

pub async fn apply_plan_changes(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/apply-plan-changes",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_athlete_settings_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/settings/desktop"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "theme": "dark",
                "units": "metric"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let settings = get_athlete_settings(&client, "a-001", "desktop")
            .await
            .unwrap();

        assert!(settings.is_object());
    }

    #[tokio::test]
    async fn test_apply_plan_changes_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/apply-plan-changes"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "updated": 5
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = apply_plan_changes(&client, "a-001").await.unwrap();

        assert!(result.is_object());
    }

    #[tokio::test]
    async fn test_get_athlete_settings_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/settings/desktop"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_athlete_settings(&client, "a-001", "desktop").await;

        assert!(result.is_err());
    }
}
