use serde::Deserialize;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct Event {
    pub id: i32,
    #[serde(default)]
    pub start_date_local: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub uid: Option<String>,
}

pub struct ListEventsParams {
    pub oldest: Option<String>,
    pub newest: Option<String>,
    pub category: Option<String>,
    pub limit: Option<i32>,
}

pub async fn list_events(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    params: &ListEventsParams,
) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let mut url = format!("{}/api/v1/athlete/{}/events", client.base_url(), athlete_id);
    let mut query_params = Vec::new();

    if let Some(ref oldest) = params.oldest {
        query_params.push(format!("oldest={}", oldest));
    }
    if let Some(ref newest) = params.newest {
        query_params.push(format!("newest={}", newest));
    }
    if let Some(ref category) = params.category {
        query_params.push(format!("category={}", category));
    }
    if let Some(limit) = params.limit {
        query_params.push(format!("limit={}", limit));
    }

    if !query_params.is_empty() {
        url.push('?');
        url.push_str(&query_params.join("&"));
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

    let events = response.json::<Vec<Event>>().await?;
    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_events_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/events"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 101,
                    "start_date_local": "2024-01-15T08:00:00",
                    "type": "WORKOUT",
                    "category": "WORKOUT",
                    "name": "Threshold Intervals",
                    "description": "6x5min at threshold",
                    "uid": "evt-001"
                },
                {
                    "id": 102,
                    "start_date_local": "2024-01-16T07:00:00",
                    "type": "NOTE",
                    "category": "NOTE",
                    "name": "Rest day",
                    "description": "Take it easy",
                    "uid": "evt-002"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let events = list_events(
            &client,
            "12345",
            &ListEventsParams {
                oldest: None,
                newest: None,
                category: None,
                limit: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].id, 101);
        assert_eq!(events[0].category, Some("WORKOUT".to_string()));
        assert_eq!(events[1].id, 102);
    }

    #[tokio::test]
    async fn test_list_events_with_filters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/events"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 101,
                    "start_date_local": "2024-01-15T08:00:00",
                    "type": "WORKOUT",
                    "category": "WORKOUT",
                    "name": "Threshold Intervals",
                    "description": "6x5min at threshold",
                    "uid": "evt-001"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let events = list_events(
            &client,
            "12345",
            &ListEventsParams {
                oldest: Some("2024-01-01".to_string()),
                newest: Some("2024-01-31".to_string()),
                category: Some("WORKOUT".to_string()),
                limit: Some(10),
            },
        )
        .await
        .unwrap();

        assert_eq!(events.len(), 1);
    }

    #[tokio::test]
    async fn test_list_events_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/events"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let events = list_events(
            &client,
            "12345",
            &ListEventsParams {
                oldest: None,
                newest: None,
                category: None,
                limit: None,
            },
        )
        .await
        .unwrap();

        assert!(events.is_empty());
    }

    #[tokio::test]
    async fn test_list_events_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/events"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_events(
            &client,
            "12345",
            &ListEventsParams {
                oldest: None,
                newest: None,
                category: None,
                limit: None,
            },
        )
        .await;

        assert!(result.is_err());
    }
}
