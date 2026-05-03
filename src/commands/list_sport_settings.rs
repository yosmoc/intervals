use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SportSettings {
    pub id: String,
    pub activity_type: String,
    pub ftp: f64,
    pub lthr: f64,
    pub name: String,
}

pub async fn list_sport_settings(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<SportSettings>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/sport-settings",
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

    let settings = response.json::<Vec<SportSettings>>().await?;
    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_sport_settings_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/sport-settings"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "sport-001",
                    "activity_type": "Ride",
                    "ftp": 280.0,
                    "lthr": 175.0,
                    "name": "Road Cycling"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let settings = list_sport_settings(&client, "12345").await.unwrap();

        assert_eq!(settings.len(), 1);
        assert_eq!(settings[0].ftp, 280.0);
    }

    #[tokio::test]
    async fn test_list_sport_settings_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/sport-settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let settings = list_sport_settings(&client, "12345").await.unwrap();

        assert!(settings.is_empty());
    }

    #[tokio::test]
    async fn test_list_sport_settings_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/sport-settings"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_sport_settings(&client, "12345").await;

        assert!(result.is_err());
    }
}
