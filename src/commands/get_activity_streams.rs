use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityStream {
    #[serde(rename = "type")]
    pub stream_type: String,
    #[serde(default)]
    pub name: Option<String>,
    pub data: Vec<serde_json::Value>,
}

pub async fn get_activity_streams(
    client: &crate::client::ApiClient,
    activity_id: &str,
    types: Option<&[String]>,
    include_defaults: bool,
) -> Result<Vec<ActivityStream>, Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/activity/{}/streams.json",
        client.base_url(),
        activity_id
    );
    let mut params = Vec::new();
    if let Some(t) = types {
        for ty in t {
            params.push(format!("types={}", urlencoding::encode(ty)));
        }
    }
    if include_defaults {
        params.push("includeDefaults=true".to_string());
    }
    if !params.is_empty() {
        url.push('?');
        url.push_str(&params.join("&"));
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

    let streams = response.json::<Vec<ActivityStream>>().await?;
    Ok(streams)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_activity_streams_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/streams.json"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "type": "heartrate",
                    "data": [120.0, 125.0, 130.0]
                },
                {
                    "type": "cadence",
                    "data": [80.0, 82.0, 85.0]
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let streams = get_activity_streams(&client, "act-001", Some(&["heartrate".into()]), false)
            .await
            .unwrap();

        assert_eq!(streams.len(), 2);
        assert_eq!(streams[0].stream_type, "heartrate");
        assert_eq!(streams[0].data.len(), 3);
    }

    #[tokio::test]
    async fn test_get_activity_streams_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/streams.json"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_activity_streams(&client, "act-001", None, false).await;

        assert!(result.is_err());
    }
}
