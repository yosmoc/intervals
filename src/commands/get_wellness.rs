use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WellnessRecord {
    pub id: Option<String>,
    pub ctl: Option<f64>,
    pub atl: Option<f64>,
    pub ramp_rate: Option<f64>,
    pub weight: Option<f64>,
    pub resting_hr: Option<i64>,
    pub hrv: Option<f64>,
    pub mood: Option<i64>,
    pub fatigue: Option<i64>,
    pub motivation: Option<i64>,
    pub sleep: Option<f64>,
    pub sleep_quality: Option<i64>,
    pub soreness: Option<i64>,
    pub stress: Option<i64>,
    pub fitness: Option<i64>,
    pub readiness: Option<i64>,
}

pub async fn get_wellness(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    date: &str,
) -> Result<WellnessRecord, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/wellness/{}",
        client.base_url(),
        athlete_id,
        date
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

    let record = response.json::<WellnessRecord>().await?;
    Ok(record)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_wellness_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/wellness/2024-01-15"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "2024-01-15",
                "ctl": 55.0,
                "atl": 50.0,
                "weight": 75.0,
                "restingHR": 60,
                "hrv": 40.0
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let record = get_wellness(&client, "a-001", "2024-01-15").await.unwrap();

        assert_eq!(record.id.as_deref(), Some("2024-01-15"));
        assert_eq!(record.ctl, Some(55.0));
    }

    #[tokio::test]
    async fn test_get_wellness_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/wellness/2024-01-15"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_wellness(&client, "a-001", "2024-01-15").await;

        assert!(result.is_err());
    }
}
