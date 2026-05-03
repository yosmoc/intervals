use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IntervalSearchResult {
    pub id: Option<String>,
    pub start_date_local: Option<String>,
    #[serde(rename = "type")]
    pub activity_type: Option<String>,
    pub name: Option<String>,
    pub distance: Option<f64>,
    pub elapsed_time: Option<i64>,
}

pub struct IntervalSearchParams {
    pub min_secs: i32,
    pub max_secs: i32,
    pub min_intensity: i32,
    pub max_intensity: i32,
    pub interval_type: Option<String>,
    pub min_reps: Option<i32>,
    pub max_reps: Option<i32>,
    pub limit: Option<i32>,
}

pub async fn search_activity_intervals(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    params: &IntervalSearchParams,
) -> Result<Vec<IntervalSearchResult>, Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/athlete/{}/activities/interval-search?minSecs={}&maxSecs={}&minIntensity={}&maxIntensity={}",
        client.base_url(),
        athlete_id,
        params.min_secs,
        params.max_secs,
        params.min_intensity,
        params.max_intensity
    );
    if let Some(ref t) = params.interval_type {
        url.push_str(&format!("&type={}", t));
    }
    if let Some(r) = params.min_reps {
        url.push_str(&format!("&minReps={}", r));
    }
    if let Some(r) = params.max_reps {
        url.push_str(&format!("&maxReps={}", r));
    }
    if let Some(l) = params.limit {
        url.push_str(&format!("&limit={}", l));
    }

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

    let results = response.json::<Vec<IntervalSearchResult>>().await?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_search_activity_intervals_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/activities/interval-search"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "i1",
                    "start_date_local": "2024-01-15T10:00:00",
                    "type": "Run",
                    "name": "Interval Run",
                    "distance": 10000.0,
                    "elapsed_time": 3000
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let params = IntervalSearchParams {
            min_secs: 60,
            max_secs: 300,
            min_intensity: 80,
            max_intensity: 100,
            interval_type: Some("HR".to_string()),
            min_reps: Some(3),
            max_reps: None,
            limit: Some(10),
        };
        let results = search_activity_intervals(&client, "a-001", &params)
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name.as_deref(), Some("Interval Run"));
    }

    #[tokio::test]
    async fn test_search_activity_intervals_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/activities/interval-search"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let params = IntervalSearchParams {
            min_secs: 60,
            max_secs: 300,
            min_intensity: 80,
            max_intensity: 100,
            interval_type: None,
            min_reps: None,
            max_reps: None,
            limit: None,
        };
        let result = search_activity_intervals(&client, "a-001", &params).await;

        assert!(result.is_err());
    }
}
