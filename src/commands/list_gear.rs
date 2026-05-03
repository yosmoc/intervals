use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Gear {
    pub id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub gear_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub retired: Option<bool>,
    #[serde(default)]
    pub athlete_id: Option<String>,
}

pub async fn list_gear(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<Gear>, Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/athlete/{}/gear", client.base_url(), athlete_id);

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

    let gear = response.json::<Vec<Gear>>().await?;
    Ok(gear)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_gear_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/gear"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "gear-001",
                    "type": "Bike",
                    "name": "Road Bike",
                    "distance": 5000000.0,
                    "retired": false,
                    "athlete_id": "12345"
                },
                {
                    "id": "gear-002",
                    "type": "Shoes",
                    "name": "Running Shoes",
                    "distance": 500000.0,
                    "retired": false,
                    "athlete_id": "12345"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let gear = list_gear(&client, "12345").await.unwrap();

        assert_eq!(gear.len(), 2);
        assert_eq!(gear[0].id, "gear-001");
        assert_eq!(gear[0].gear_type, Some("Bike".to_string()));
        assert_eq!(gear[1].id, "gear-002");
    }

    #[tokio::test]
    async fn test_list_gear_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/gear"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let gear = list_gear(&client, "12345").await.unwrap();

        assert!(gear.is_empty());
    }

    #[tokio::test]
    async fn test_list_gear_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/gear"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_gear(&client, "12345").await;

        assert!(result.is_err());
    }
}
