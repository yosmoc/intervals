use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerCurve {
    pub id: String,
    pub duration: f64,
    pub power: f64,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerCurveResponse {
    #[serde(default)]
    pub list: Vec<PowerCurve>,
}

pub async fn list_athlete_power_curves(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    activity_type: &str,
) -> Result<Vec<PowerCurve>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/power-curves?type={}",
        client.base_url(),
        athlete_id,
        activity_type
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

    let wrapper = response.json::<PowerCurveResponse>().await?;
    Ok(wrapper.list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_athlete_power_curves_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/power-curves"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "list": [
                    {
                        "id": "pc-001",
                        "duration": 60.0,
                        "power": 350.0,
                        "date": "2024-01-15"
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let curves = list_athlete_power_curves(&client, "12345", "Ride").await.unwrap();

        assert_eq!(curves.len(), 1);
        assert_eq!(curves[0].power, 350.0);
    }

    #[tokio::test]
    async fn test_list_athlete_power_curves_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/power-curves"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_athlete_power_curves(&client, "12345", "Ride").await;

        assert!(result.is_err());
    }
}
