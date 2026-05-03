use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomItem {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub item_type: Option<String>,
    pub config: Option<serde_json::Value>,
}

pub async fn list_custom_items(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<CustomItem>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/custom-item",
        client.base_url(),
        athlete_id
    );
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

    let items = response.json::<Vec<CustomItem>>().await?;
    Ok(items)
}

pub async fn get_custom_item(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    item_id: i64,
) -> Result<CustomItem, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/custom-item/{}",
        client.base_url(),
        athlete_id,
        item_id
    );
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

    let item = response.json::<CustomItem>().await?;
    Ok(item)
}

pub async fn delete_custom_item(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    item_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/custom-item/{}",
        client.base_url(),
        athlete_id,
        item_id
    );
    let response = client
        .client()
        .delete(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
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
    async fn test_list_custom_items_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/custom-item"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 1,
                    "name": "Custom Chart",
                    "item_type": "CHART"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let items = list_custom_items(&client, "a-001").await.unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name.as_deref(), Some("Custom Chart"));
    }

    #[tokio::test]
    async fn test_delete_custom_item_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/athlete/a-001/custom-item/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_custom_item(&client, "a-001", 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_custom_items_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/custom-item"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_custom_items(&client, "a-001").await;

        assert!(result.is_err());
    }
}
