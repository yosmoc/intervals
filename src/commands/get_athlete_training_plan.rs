use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TrainingPlan {
    pub id: String,
    pub name: String,
    pub description: String,
    pub start_date: String,
    pub weeks: i32,
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
                "id": "plan-001",
                "name": "Base Training",
                "description": "12 week base training plan",
                "start_date": "2024-01-01",
                "weeks": 12
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let plan = get_athlete_training_plan(&client, "12345").await.unwrap();

        assert_eq!(plan.id, "plan-001");
        assert_eq!(plan.weeks, 12);
    }

    #[tokio::test]
    async fn test_get_athlete_training_plan_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/99999/training-plan"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({
                        "error": "Training plan not found"
                    })),
            )
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
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_athlete_training_plan(&client, "12345").await;

        assert!(result.is_err());
    }
}
