use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BestEffort {
    pub start_index: i64,
    pub end_index: i64,
    pub average: f64,
    pub duration: i64,
    pub distance: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BestEfforts {
    #[serde(default)]
    pub efforts: Vec<BestEffort>,
}

pub struct BestEffortsParams {
    pub stream: String,
    pub duration: Option<i64>,
    pub distance: Option<f64>,
    pub count: Option<i64>,
    pub min_value: Option<f64>,
    pub exclude_intervals: Option<bool>,
    pub start_index: Option<i64>,
    pub end_index: Option<i64>,
}

pub async fn get_activity_best_efforts(
    client: &crate::client::ApiClient,
    activity_id: &str,
    params: &BestEffortsParams,
) -> Result<BestEfforts, Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/activity/{}/best-efforts?stream={}",
        client.base_url(),
        activity_id,
        urlencoding::encode(&params.stream)
    );
    if let Some(d) = params.duration {
        url.push_str(&format!("&duration={}", d));
    }
    if let Some(d) = params.distance {
        url.push_str(&format!("&distance={}", d));
    }
    if let Some(c) = params.count {
        url.push_str(&format!("&count={}", c));
    }
    if let Some(v) = params.min_value {
        url.push_str(&format!("&minValue={}", v));
    }
    if let Some(e) = params.exclude_intervals {
        url.push_str(&format!("&excludeIntervals={}", e));
    }
    if let Some(s) = params.start_index {
        url.push_str(&format!("&startIndex={}", s));
    }
    if let Some(e) = params.end_index {
        url.push_str(&format!("&endIndex={}", e));
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

    let result = response.json::<BestEfforts>().await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_activity_best_efforts_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/best-efforts"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "efforts": [
                    {
                        "start_index": 0,
                        "end_index": 60,
                        "average": 350.0,
                        "duration": 60,
                        "distance": 500.0
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let params = BestEffortsParams {
            stream: "watts".to_string(),
            duration: Some(60),
            distance: None,
            count: None,
            min_value: None,
            exclude_intervals: None,
            start_index: None,
            end_index: None,
        };
        let result = get_activity_best_efforts(&client, "act-001", &params)
            .await
            .unwrap();

        assert_eq!(result.efforts.len(), 1);
        assert_eq!(result.efforts[0].average, 350.0);
    }

    #[tokio::test]
    async fn test_get_activity_best_efforts_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/best-efforts"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let params = BestEffortsParams {
            stream: "watts".to_string(),
            duration: Some(60),
            distance: None,
            count: None,
            min_value: None,
            exclude_intervals: None,
            start_index: None,
            end_index: None,
        };
        let result = get_activity_best_efforts(&client, "act-001", &params).await;

        assert!(result.is_err());
    }
}
