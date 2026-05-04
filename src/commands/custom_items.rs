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

pub async fn create_custom_item(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &CustomItem,
) -> Result<CustomItem, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/custom-item",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(input)
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

pub async fn update_custom_item(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    item_id: i64,
    input: &CustomItem,
) -> Result<CustomItem, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/custom-item/{}",
        client.base_url(),
        athlete_id,
        item_id
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

    let item = response.json::<CustomItem>().await?;
    Ok(item)
}

pub async fn update_custom_item_indexes(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    indexes: &[(i64, i32)],
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/custom-item-indexes",
        client.base_url(),
        athlete_id
    );
    let body: Vec<serde_json::Value> = indexes
        .iter()
        .map(|(id, idx)| serde_json::json!({ "id": id, "index": idx }))
        .collect();
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body_text = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body_text).into());
    }

    Ok(())
}

pub async fn upload_custom_item_image(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    item_id: i64,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/custom-item/{}/image",
        client.base_url(),
        athlete_id,
        item_id
    );
    let file_bytes = std::fs::read(file_path)?;
    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .header("Content-Type", "application/octet-stream")
        .body(file_bytes)
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

    #[tokio::test]
    async fn test_create_custom_item_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/custom-item"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 2,
                "name": "New Item",
                "item_type": "CHART"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CustomItem {
            id: None,
            name: Some("New Item".to_string()),
            item_type: Some("CHART".to_string()),
            config: None,
        };
        let result = create_custom_item(&client, "a-001", &input).await.unwrap();

        assert_eq!(result.name.as_deref(), Some("New Item"));
    }

    #[tokio::test]
    async fn test_update_custom_item_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/custom-item/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "name": "Updated Item"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CustomItem {
            id: Some(1),
            name: Some("Updated Item".to_string()),
            item_type: None,
            config: None,
        };
        let result = update_custom_item(&client, "a-001", 1, &input)
            .await
            .unwrap();

        assert_eq!(result.name.as_deref(), Some("Updated Item"));
    }

    #[tokio::test]
    async fn test_update_custom_item_indexes_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/custom-item-indexes"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = update_custom_item_indexes(&client, "a-001", &[(1, 0), (2, 1)]).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_upload_custom_item_image_success() {
        let mock_server = MockServer::start().await;
        let temp_file = std::env::temp_dir().join("test_image.png");
        std::fs::write(&temp_file, b"fake image").unwrap();

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/custom-item/1/image"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result =
            upload_custom_item_image(&client, "a-001", 1, temp_file.to_str().unwrap()).await;

        assert!(result.is_ok());
        std::fs::remove_file(&temp_file).ok();
    }
}
