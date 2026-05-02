use serde::Deserialize;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct Athlete {
    pub id: i64,
    #[serde(default)]
    pub name: Option<String>,
}

pub async fn get_athlete(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Athlete, Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/athlete/{}", client.base_url(), athlete_id);
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

    let athlete = response.json::<Athlete>().await?;
    Ok(athlete)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_athlete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345"))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 12345,
                "name": "Test Athlete"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let athlete = get_athlete(&client, "12345").await.unwrap();

        assert_eq!(athlete.id, 12345);
        assert_eq!(athlete.name, Some("Test Athlete".to_string()));
    }

    #[tokio::test]
    async fn test_get_athlete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/99999"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({
                        "error": "Athlete not found"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_athlete(&client, "99999").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_athlete_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_athlete(&client, "12345").await;

        assert!(result.is_err());
    }

    #[test]
    fn test_api_client_from_env() {
        unsafe { std::env::remove_var("INTERVALS_API_KEY") };
        let result = ApiClient::from_env("http://localhost".to_string());
        assert!(result.is_err());

        unsafe { std::env::set_var("INTERVALS_API_KEY", "test-key") };
        let result = ApiClient::from_env("http://localhost".to_string());
        assert!(result.is_ok());
        unsafe { std::env::remove_var("INTERVALS_API_KEY") };
    }
}
