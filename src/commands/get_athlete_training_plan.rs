use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TrainingPlan {
    #[serde(default)]
    pub athlete_id: Option<String>,
    #[serde(default)]
    pub training_plan_id: Option<String>,
    #[serde(default)]
    pub training_plan_start_date: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub training_plan_last_applied: Option<String>,
    #[serde(default)]
    pub training_plan: Option<String>,
    #[serde(default)]
    pub training_plan_alias: Option<String>,
}

pub async fn get_athlete_training_plan(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<TrainingPlan, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/training-plan",
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

    let plan = response.json::<TrainingPlan>().await?;
    Ok(plan)
}

pub async fn update_training_plan(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/training-plan",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(input)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AthletePlanUpdate {
    pub athlete_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_plan_start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_plan_alias: Option<String>,
}

pub async fn update_athlete_plans(
    client: &crate::client::ApiClient,
    updates: &[AthletePlanUpdate],
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/athlete-plans", client.base_url());
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(updates)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_athlete_training_plan_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/training-plan"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "athlete_id": "i12345",
                "training_plan_id": "plan-001",
                "training_plan_start_date": "2024-01-01",
                "timezone": "Europe/Stockholm",
                "training_plan_alias": "Base Plan"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let plan = get_athlete_training_plan(&client, "12345").await.unwrap();

        assert_eq!(plan.athlete_id, Some("i12345".to_string()));
        assert_eq!(plan.training_plan_alias, Some("Base Plan".to_string()));
    }

    #[tokio::test]
    async fn test_get_athlete_training_plan_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/99999/training-plan"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "error": "Not found"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_athlete_training_plan(&client, "99999").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_athlete_training_plan_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/training-plan"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_athlete_training_plan(&client, "12345").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_training_plan_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/12345/training-plan"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = serde_json::json!({ "training_plan_id": "plan-002" });
        let result = update_training_plan(&client, "12345", &input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_athlete_plans_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete-plans"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let updates = vec![AthletePlanUpdate {
            athlete_id: "i12345".to_string(),
            training_plan_id: Some("plan-001".to_string()),
            training_plan_start_date: None,
            training_plan_alias: None,
        }];
        let result = update_athlete_plans(&client, &updates).await;

        assert!(result.is_ok());
    }
}
