use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaceCurve {
    pub id: String,
    pub label: String,
    pub days: i64,
    #[serde(default)]
    pub distance: Option<Vec<f64>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaceCurveResponse {
    #[serde(default)]
    pub list: Vec<PaceCurve>,
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

    let wrapper = response.json::<PaceCurveResponse>().await?;
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
    async fn test_list_athlete_pace_curves_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/pace-curves"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "list": [
                    {
                        "id": "1y",
                        "label": "1 year",
                        "days": 365,
                        "distance": [5.0, 10.0, 21.1]
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let curves = list_athlete_pace_curves(&client, "12345").await.unwrap();

        assert_eq!(curves.len(), 1);
        assert_eq!(curves[0].id, "1y");
    }

    #[tokio::test]
    async fn test_list_athlete_pace_curves_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/pace-curves"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_athlete_pace_curves(&client, "12345").await;

        assert!(result.is_err());
    }
}
