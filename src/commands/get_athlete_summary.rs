use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SummaryWithCats {
    pub count: Option<i64>,
    pub time: Option<i64>,
    pub moving_time: Option<i64>,
    pub elapsed_time: Option<i64>,
    pub calories: Option<i64>,
    pub total_elevation_gain: Option<f64>,
    pub training_load: Option<i64>,
    pub srpe: Option<i64>,
    pub distance: Option<f64>,
    pub eftp: Option<f64>,
    pub eftp_per_kg: Option<f64>,
    pub date: Option<String>,
    pub athlete_id: Option<String>,
    pub athlete_name: Option<String>,
    pub categories: Option<serde_json::Value>,
}

pub struct GetAthleteSummaryParams {
    pub start: Option<String>,
    pub end: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub async fn get_athlete_summary(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    params: &GetAthleteSummaryParams,
) -> Result<Vec<SummaryWithCats>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/athlete-summary.json",
        client.base_url(),
        athlete_id
    );

    let mut query_params: Vec<(&str, String)> = Vec::new();
    if let Some(ref start) = params.start {
        query_params.push(("start", start.clone()));
    }
    if let Some(ref end) = params.end {
        query_params.push(("end", end.clone()));
    }
    if let Some(ref tags) = params.tags {
        for tag in tags {
            query_params.push(("tags", tag.clone()));
        }
    }

    let response = client
        .client()
        .get(&url)
        .query(&query_params)
        .basic_auth("API_KEY", Some(client.api_key()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let summaries = response.json::<Vec<SummaryWithCats>>().await?;
    Ok(summaries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_athlete_summary_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/athlete-summary.json"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "count": 2,
                    "moving_time": 7200,
                    "distance": 50000.0,
                    "training_load": 150,
                    "date": "2024-01-15",
                    "athlete_id": "a-001",
                    "athlete_name": "Test Athlete"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let params = GetAthleteSummaryParams {
            start: Some("2024-01-10".to_string()),
            end: Some("2024-01-15".to_string()),
            tags: None,
        };
        let summaries = get_athlete_summary(&client, "a-001", &params)
            .await
            .unwrap();

        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].athlete_name.as_deref(), Some("Test Athlete"));
    }

    #[tokio::test]
    async fn test_get_athlete_summary_special_char_tags() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/athlete-summary.json"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .and(wiremock::matchers::query_param("tags", "#training"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let params = GetAthleteSummaryParams {
            start: None,
            end: None,
            tags: Some(vec!["#training".to_string()]),
        };
        let result = get_athlete_summary(&client, "a-001", &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_athlete_summary_tag_with_space() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/athlete-summary.json"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .and(wiremock::matchers::query_param("tags", "tempo run"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let params = GetAthleteSummaryParams {
            start: None,
            end: None,
            tags: Some(vec!["tempo run".to_string()]),
        };
        let result = get_athlete_summary(&client, "a-001", &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_athlete_summary_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/athlete-summary.json"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let params = GetAthleteSummaryParams {
            start: None,
            end: None,
            tags: None,
        };
        let result = get_athlete_summary(&client, "a-001", &params).await;

        assert!(result.is_err());
    }
}
