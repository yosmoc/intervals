use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityInterval {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub start_index: Option<i64>,
    #[serde(default)]
    pub end_index: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub intensity: Option<f64>,
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub laps: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateIntervalsInput {
    pub intervals: Vec<ActivityInterval>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateIntervalInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intensity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub laps: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SplitIntervalInput {
    pub activity_id: String,
    pub interval_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_index: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteIntervalsInput {
    pub interval_ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hr: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watts: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cadence: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latlng: Option<Vec<Vec<f64>>>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<serde_json::Value>,
}

pub async fn update_activity_intervals(
    client: &crate::client::ApiClient,
    activity_id: &str,
    input: &UpdateIntervalsInput,
) -> Result<Vec<ActivityInterval>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/intervals",
        client.base_url(),
        activity_id
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

    let result = response.json::<Vec<ActivityInterval>>().await?;
    Ok(result)
}

pub async fn update_activity_interval(
    client: &crate::client::ApiClient,
    activity_id: &str,
    interval_id: i64,
    input: &UpdateIntervalInput,
) -> Result<ActivityInterval, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/intervals/{}",
        client.base_url(),
        activity_id,
        interval_id
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

    let result = response.json::<ActivityInterval>().await?;
    Ok(result)
}

pub async fn split_activity_interval(
    client: &crate::client::ApiClient,
    activity_id: &str,
    input: &SplitIntervalInput,
) -> Result<Vec<ActivityInterval>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/split-interval",
        client.base_url(),
        activity_id
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

    let result = response.json::<Vec<ActivityInterval>>().await?;
    Ok(result)
}

pub async fn delete_activity_intervals(
    client: &crate::client::ApiClient,
    activity_id: &str,
    input: &DeleteIntervalsInput,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/delete-intervals",
        client.base_url(),
        activity_id
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

pub async fn update_activity_streams(
    client: &crate::client::ApiClient,
    activity_id: &str,
    input: &StreamUpdate,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/streams",
        client.base_url(),
        activity_id
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

pub async fn update_activity_streams_csv(
    client: &crate::client::ApiClient,
    activity_id: &str,
    csv_content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/streams.csv",
        client.base_url(),
        activity_id
    );

    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .header("Content-Type", "text/csv")
        .body(csv_content.to_string())
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
    async fn test_update_activity_intervals_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/activity/act-001/intervals"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 1, "name": "Warmup", "start_index": 0, "end_index": 60}
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = UpdateIntervalsInput {
            intervals: vec![ActivityInterval {
                id: Some(1),
                start_index: Some(0),
                end_index: Some(60),
                name: Some("Warmup".to_string()),
                intensity: None,
                duration: None,
                distance: None,
                laps: None,
            }],
        };
        let result = update_activity_intervals(&client, "act-001", &input)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, Some("Warmup".to_string()));
    }

    #[tokio::test]
    async fn test_update_activity_intervals_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/activity/act-001/intervals"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let input = UpdateIntervalsInput { intervals: vec![] };
        let result = update_activity_intervals(&client, "act-001", &input).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_activity_interval_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/activity/act-001/intervals/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1, "name": "Updated Interval", "start_index": 0, "end_index": 120
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = UpdateIntervalInput {
            id: Some(1),
            start_index: Some(0),
            end_index: Some(120),
            name: Some("Updated Interval".to_string()),
            intensity: None,
            duration: None,
            distance: None,
            laps: None,
        };
        let result = update_activity_interval(&client, "act-001", 1, &input)
            .await
            .unwrap();

        assert_eq!(result.name, Some("Updated Interval".to_string()));
    }

    #[tokio::test]
    async fn test_split_activity_interval_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/activity/act-001/split-interval"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 1, "name": "Part 1"},
                {"id": 2, "name": "Part 2"}
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = SplitIntervalInput {
            activity_id: "act-001".to_string(),
            interval_id: 1,
            split_index: Some(30),
        };
        let result = split_activity_interval(&client, "act-001", &input)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_activity_intervals_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/activity/act-001/delete-intervals"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = DeleteIntervalsInput {
            interval_ids: vec![1, 2],
        };
        let result = delete_activity_intervals(&client, "act-001", &input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_activity_streams_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/activity/act-001/streams"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = StreamUpdate {
            time: Some(vec![0.0, 1.0, 2.0]),
            hr: Some(vec![120.0, 125.0, 130.0]),
            watts: None,
            cadence: None,
            distance: None,
            altitude: None,
            latlng: None,
            other: None,
        };
        let result = update_activity_streams(&client, "act-001", &input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_activity_streams_csv_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/activity/act-001/streams.csv"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let csv = "time,hr,watts\n0,120,200\n1,125,210";
        let result = update_activity_streams_csv(&client, "act-001", csv).await;

        assert!(result.is_ok());
    }
}
