use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AthleteProfile {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub dob: String,
    pub weight: f64,
    pub gender: String,
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
        .header("Authorization", format!("Bearer {}", client.api_key()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let profile = response.json::<AthleteProfile>().await?;
    Ok(profile)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_athlete_profile_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/profile"))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 12345,
                "name": "Test Athlete",
                "email": "test@example.com",
                "dob": "1990-01-01",
                "weight": 75.0,
                "gender": "M"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let profile = get_athlete_profile(&client, "12345").await.unwrap();

        assert_eq!(profile.id, 12345);
        assert_eq!(profile.name, "Test Athlete");
        assert_eq!(profile.email, "test@example.com");
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
