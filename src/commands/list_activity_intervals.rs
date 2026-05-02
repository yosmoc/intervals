use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Interval {
    pub id: String,
    pub start_offset: f64,
    pub stop_offset: f64,
    pub avg_power: f64,
    pub avg_hr: f64,
}

pub async fn list_activity_intervals(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<Vec<Interval>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/intervals",
        client.base_url(),
        activity_id
    );
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

    let intervals = response.json::<Vec<Interval>>().await?;
    Ok(intervals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_activity_intervals_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/intervals"))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "int-001",
                    "start_offset": 300.0,
                    "stop_offset": 600.0,
                    "avg_power": 320.0,
                    "avg_hr": 165.0
                },
                {
                    "id": "int-002",
                    "start_offset": 900.0,
                    "stop_offset": 1200.0,
                    "avg_power": 330.0,
                    "avg_hr": 168.0
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let intervals = list_activity_intervals(&client, "act-001").await.unwrap();

        assert_eq!(intervals.len(), 2);
        assert_eq!(intervals[0].avg_power, 320.0);
    }

    #[tokio::test]
    async fn test_list_activity_intervals_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/intervals"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let intervals = list_activity_intervals(&client, "act-001").await.unwrap();

        assert!(intervals.is_empty());
    }

    #[tokio::test]
    async fn test_list_activity_intervals_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/intervals"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_activity_intervals(&client, "act-001").await;

        assert!(result.is_err());
    }
}
