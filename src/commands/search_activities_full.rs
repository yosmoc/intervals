use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FullActivity {
    pub id: Option<String>,
    pub start_date_local: Option<String>,
    #[serde(rename = "type")]
    pub activity_type: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub distance: Option<f64>,
    pub elapsed_time: Option<i64>,
    pub moving_time: Option<i64>,
    pub total_elevation_gain: Option<f64>,
    pub average_speed: Option<f64>,
    pub max_speed: Option<f64>,
    pub average_heartrate: Option<f64>,
    pub max_heartrate: Option<i64>,
    pub calories: Option<f64>,
    pub tags: Option<Vec<String>>,
}

pub async fn search_activities_full(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    query: &str,
    limit: Option<i32>,
) -> Result<Vec<FullActivity>, Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/athlete/{}/activities/search-full?q={}",
        client.base_url(),
        athlete_id,
        urlencoding::encode(query)
    );
    if let Some(l) = limit {
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

    let activities = response.json::<Vec<FullActivity>>().await?;
    Ok(activities)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_search_activities_full_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/activities/search-full"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "i1",
                    "start_date_local": "2024-01-15T10:00:00",
                    "type": "Run",
                    "name": "Morning Run",
                    "distance": 5000.0,
                    "elapsed_time": 1500,
                    "tags": ["tempo"]
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let activities = search_activities_full(&client, "a-001", "Morning", Some(10))
            .await
            .unwrap();

        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0].name.as_deref(), Some("Morning Run"));
    }

    #[tokio::test]
    async fn test_search_activities_full_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/activities/search-full"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = search_activities_full(&client, "a-001", "Run", None).await;

        assert!(result.is_err());
    }
}
