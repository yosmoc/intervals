use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerModel {
    pub model_type: Option<String>,
    pub parameters: Option<serde_json::Value>,
}

pub async fn get_athlete_mmp_model(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    activity_type: &str,
) -> Result<PowerModel, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/mmp-model?type={}",
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

    let model = response.json::<PowerModel>().await?;
    Ok(model)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerHRCurve {
    pub data: Option<Vec<serde_json::Value>>,
}

pub async fn get_power_hr_curve(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    start: &str,
    end: &str,
) -> Result<PowerHRCurve, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/power-hr-curve?start={}&end={}",
        client.base_url(),
        athlete_id,
        start,
        end
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

    let curve = response.json::<PowerHRCurve>().await?;
    Ok(curve)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_athlete_mmp_model_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/mmp-model"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "model_type": "Ride",
                "parameters": {}
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let model = get_athlete_mmp_model(&client, "a-001", "Ride")
            .await
            .unwrap();

        assert_eq!(model.model_type.as_deref(), Some("Ride"));
    }

    #[tokio::test]
    async fn test_get_power_hr_curve_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/power-hr-curve"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": []
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let curve = get_power_hr_curve(&client, "a-001", "2024-01-01", "2024-01-31")
            .await
            .unwrap();

        assert!(curve.data.is_some());
    }

    #[tokio::test]
    async fn test_get_athlete_mmp_model_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/mmp-model"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_athlete_mmp_model(&client, "a-001", "Ride").await;

        assert!(result.is_err());
    }
}
