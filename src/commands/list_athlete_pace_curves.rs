use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaceCurve {
    pub distance: f64,
    pub pace: f64,
    pub date: String,
}

pub async fn list_athlete_pace_curves(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<PaceCurve>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/pace-curves",
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

    let curves = response.json::<Vec<PaceCurve>>().await?;
    Ok(curves)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_athlete_pace_curves_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/pace-curves"))
            .and(header("Authorization", "Basic QVBJX0tFWTp0ZXN0LWFwaS1rZXk="))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "distance": 1000.0,
                    "pace": 240.0,
                    "date": "2024-01-15"
                },
                {
                    "distance": 5000.0,
                    "pace": 260.0,
                    "date": "2024-01-14"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let curves = list_athlete_pace_curves(&client, "12345").await.unwrap();

        assert_eq!(curves.len(), 2);
        assert_eq!(curves[0].distance, 1000.0);
    }

    #[tokio::test]
    async fn test_list_athlete_pace_curves_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/pace-curves"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_athlete_pace_curves(&client, "12345").await;

        assert!(result.is_err());
    }
}
