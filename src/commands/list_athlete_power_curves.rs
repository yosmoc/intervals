use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerCurve {
    pub id: String,
    pub duration: f64,
    pub power: f64,
    pub date: String,
}

pub async fn list_athlete_power_curves(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<PowerCurve>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/power-curves",
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

    let curves = response.json::<Vec<PowerCurve>>().await?;
    Ok(curves)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_athlete_power_curves_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/power-curves"))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "pc-001",
                    "duration": 60.0,
                    "power": 350.0,
                    "date": "2024-01-15"
                },
                {
                    "id": "pc-002",
                    "duration": 300.0,
                    "power": 300.0,
                    "date": "2024-01-14"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let curves = list_athlete_power_curves(&client, "12345").await.unwrap();

        assert_eq!(curves.len(), 2);
        assert_eq!(curves[0].id, "pc-001");
        assert_eq!(curves[0].power, 350.0);
    }

    #[tokio::test]
    async fn test_list_athlete_power_curves_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/power-curves"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_athlete_power_curves(&client, "12345").await;

        assert!(result.is_err());
    }
}
