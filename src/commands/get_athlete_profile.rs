use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AthleteProfile {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub sex: Option<String>,
}

pub async fn get_athlete_profile(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<AthleteProfile, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/profile",
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

    let json: serde_json::Value = response.json().await?;
    let athlete = json.get("athlete").ok_or("Missing 'athlete' field")?;
    let profile = serde_json::from_value(athlete.clone())?;
    Ok(profile)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_athlete_profile_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/profile"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "athlete": {
                    "id": "i12345",
                    "name": "Test Athlete",
                    "city": "Lund",
                    "country": "Sweden",
                    "timezone": "Europe/Stockholm",
                    "sex": "M"
                }
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let profile = get_athlete_profile(&client, "12345").await.unwrap();

        assert_eq!(profile.id, "i12345");
        assert_eq!(profile.name, "Test Athlete");
    }

    #[tokio::test]
    async fn test_get_athlete_profile_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/99999/profile"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({
                        "error": "Athlete not found"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_athlete_profile(&client, "99999").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_athlete_profile_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/profile"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_athlete_profile(&client, "12345").await;

        assert!(result.is_err());
    }
}
