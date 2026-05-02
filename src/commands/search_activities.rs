use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivitySearchResult {
    pub id: String,
    #[serde(default)]
    pub start_date_local: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub activity_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub elapsed_time: Option<i64>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

pub struct SearchActivitiesParams {
    pub query: String,
    pub limit: Option<i32>,
    pub full: bool,
}

pub async fn search_activities(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    params: &SearchActivitiesParams,
) -> Result<Vec<ActivitySearchResult>, Box<dyn std::error::Error>> {
    let endpoint = if params.full {
        "search-full"
    } else {
        "search"
    };
    let mut url = format!(
        "{}/api/v1/athlete/{}/activities/{}?q={}",
        client.base_url(),
        athlete_id,
        endpoint,
        urlencoding::encode(&params.query)
    );

    if let Some(limit) = params.limit {
        url.push_str(&format!("&limit={}", limit));
    }

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

    let results = response.json::<Vec<ActivitySearchResult>>().await?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path_regex, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_search_activities_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/activities/search"))
            .and(query_param("q", "morning"))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "act-001",
                    "start_date_local": "2024-01-15T08:00:00",
                    "type": "Ride",
                    "name": "Morning Ride",
                    "distance": 25000.0,
                    "elapsed_time": 3600,
                    "tags": ["training"]
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let results = search_activities(&client, "12345", &SearchActivitiesParams {
            query: "morning".to_string(),
            limit: None,
            full: false,
        }).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "act-001");
        assert_eq!(results[0].name, Some("Morning Ride".to_string()));
    }

    #[tokio::test]
    async fn test_search_activities_with_tag() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/activities/search"))
            .and(query_param("q", "#training"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "act-002",
                    "start_date_local": "2024-01-16T07:00:00",
                    "type": "Run",
                    "name": "Training Run",
                    "distance": 10000.0,
                    "elapsed_time": 3000,
                    "tags": ["training", "easy"]
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let results = search_activities(&client, "12345", &SearchActivitiesParams {
            query: "#training".to_string(),
            limit: None,
            full: false,
        }).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].tags, Some(vec!["training".to_string(), "easy".to_string()]));
    }

    #[tokio::test]
    async fn test_search_activities_with_limit() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/activities/search"))
            .and(query_param("q", "ride"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let results = search_activities(&client, "12345", &SearchActivitiesParams {
            query: "ride".to_string(),
            limit: Some(5),
            full: false,
        }).await.unwrap();

        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_search_activities_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/activities/search"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = search_activities(&client, "12345", &SearchActivitiesParams {
            query: "test".to_string(),
            limit: None,
            full: false,
        }).await;

        assert!(result.is_err());
    }
}
