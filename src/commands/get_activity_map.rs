use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MapData {
    pub latlngs: Option<Vec<Option<Vec<f64>>>>,
    pub bounds: Option<Vec<Vec<f64>>>,
}

pub async fn get_activity_map(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<MapData, Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/activity/{}/map", client.base_url(), activity_id);
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

    let map = response.json::<MapData>().await?;
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_activity_map_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/map"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "latlngs": [[37.7749, -122.4194], [37.7750, -122.4195]],
                "bounds": [[37.7749, -122.4194], [37.7750, -122.4195]]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let map = get_activity_map(&client, "act-001").await.unwrap();

        assert!(map.latlngs.is_some());
        assert_eq!(map.latlngs.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_get_activity_map_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/map"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_activity_map(&client, "act-001").await;

        assert!(result.is_err());
    }
}
