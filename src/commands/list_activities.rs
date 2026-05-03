use serde::Deserialize;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct ActivitySummary {
    pub id: String,
    #[serde(default)]
    pub start_date_local: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub activity_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub elapsed_time: Option<i64>,
    #[serde(default)]
    pub distance: Option<f64>,
}

pub struct ListActivitiesParams {
    pub oldest: String,
    pub newest: Option<String>,
    pub route_id: Option<i64>,
    pub limit: Option<i32>,
}

pub async fn list_activities(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    params: &ListActivitiesParams,
) -> Result<Vec<ActivitySummary>, Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/athlete/{}/activities",
        client.base_url(),
        athlete_id
    );
    let mut query_params = vec![format!("oldest={}", params.oldest)];

    if let Some(ref newest) = params.newest {
        query_params.push(format!("newest={}", newest));
    }
    if let Some(route_id) = params.route_id {
        query_params.push(format!("route_id={}", route_id));
    }
    if let Some(limit) = params.limit {
        query_params.push(format!("limit={}", limit));
    }

    url.push('?');
    url.push_str(&query_params.join("&"));

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

    let activities = response.json::<Vec<ActivitySummary>>().await?;
    Ok(activities)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_activities_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/activities"))
            .and(header("Authorization", "Basic QVBJX0tFWTp0ZXN0LWFwaS1rZXk="))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "act-001",
                    "start_date_local": "2024-01-15T08:00:00",
                    "type": "Ride",
                    "name": "Morning Ride",
                    "elapsed_time": 3600,
                    "distance": 25000.0
                },
                {
                    "id": "act-002",
                    "start_date_local": "2024-01-14T07:30:00",
                    "type": "Run",
                    "name": "Easy Run",
                    "elapsed_time": 1800,
                    "distance": 8000.0
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let activities = list_activities(&client, "12345", &ListActivitiesParams {
            oldest: "2024-01-01".to_string(),
            newest: None,
            route_id: None,
            limit: None,
        }).await.unwrap();

        assert_eq!(activities.len(), 2);
        assert_eq!(activities[0].id, "act-001");
        assert_eq!(activities[0].activity_type, Some("Ride".to_string()));
        assert_eq!(activities[1].id, "act-002");
    }

    #[tokio::test]
    async fn test_list_activities_with_filters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/activities"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "act-001",
                    "start_date_local": "2024-01-15T08:00:00",
                    "type": "Ride",
                    "name": "Morning Ride",
                    "elapsed_time": 3600,
                    "distance": 25000.0
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let activities = list_activities(&client, "12345", &ListActivitiesParams {
            oldest: "2024-01-01".to_string(),
            newest: Some("2024-01-31".to_string()),
            route_id: Some(42),
            limit: Some(10),
        }).await.unwrap();

        assert_eq!(activities.len(), 1);
    }

    #[tokio::test]
    async fn test_list_activities_empty_result() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/activities"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let activities = list_activities(&client, "12345", &ListActivitiesParams {
            oldest: "2024-01-01".to_string(),
            newest: None,
            route_id: None,
            limit: None,
        }).await.unwrap();

        assert!(activities.is_empty());
    }

    #[tokio::test]
    async fn test_list_activities_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/activities"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_activities(&client, "12345", &ListActivitiesParams {
            oldest: "2024-01-01".to_string(),
            newest: None,
            route_id: None,
            limit: None,
        }).await;

        assert!(result.is_err());
    }
}
