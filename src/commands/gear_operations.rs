use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GearItem {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub retired: Option<bool>,
    #[serde(default)]
    pub retired_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GearStats {
    pub distance: Option<f64>,
    pub time: Option<f64>,
    pub activities: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateGearInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retired: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReplaceGearInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_gear_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retired_date: Option<String>,
}

pub async fn get_gear(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    gear_id: &str,
) -> Result<GearItem, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/gear/{}",
        client.base_url(),
        athlete_id,
        gear_id
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

    let gear = response.json::<GearItem>().await?;
    Ok(gear)
}

pub async fn create_gear(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &CreateGearInput,
) -> Result<GearItem, Box<dyn std::error::Error>> {
    let url = format!("{}/api/v1/athlete/{}/gear", client.base_url(), athlete_id);
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

    let gear = response.json::<GearItem>().await?;
    Ok(gear)
}

pub async fn update_gear(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    gear_id: &str,
    input: &CreateGearInput,
) -> Result<GearItem, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/gear/{}",
        client.base_url(),
        athlete_id,
        gear_id
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

    let gear = response.json::<GearItem>().await?;
    Ok(gear)
}

pub async fn delete_gear(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    gear_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/gear/{}",
        client.base_url(),
        athlete_id,
        gear_id
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

pub async fn replace_gear(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    gear_id: &str,
    input: &ReplaceGearInput,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/gear/{}/replace",
        client.base_url(),
        athlete_id,
        gear_id
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

    Ok(())
}

pub async fn calc_gear(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    gear_id: &str,
) -> Result<GearStats, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/gear/{}/calc",
        client.base_url(),
        athlete_id,
        gear_id
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

    let stats = response.json::<GearStats>().await?;
    Ok(stats)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GearReminder {
    pub id: Option<i64>,
    pub distance: Option<f64>,
    pub date: Option<String>,
    pub notes: Option<String>,
    pub done: Option<bool>,
}

pub async fn create_gear_reminder(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    gear_id: &str,
    reminder: &GearReminder,
) -> Result<GearReminder, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/gear/{}/reminder",
        client.base_url(),
        athlete_id,
        gear_id
    );
    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(reminder)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let result = response.json::<GearReminder>().await?;
    Ok(result)
}

pub async fn update_gear_reminder(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    gear_id: &str,
    reminder_id: i64,
    reminder: &GearReminder,
) -> Result<GearReminder, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/gear/{}/reminder/{}",
        client.base_url(),
        athlete_id,
        gear_id,
        reminder_id
    );
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(reminder)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let result = response.json::<GearReminder>().await?;
    Ok(result)
}

pub async fn delete_gear_reminder(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    gear_id: &str,
    reminder_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/gear/{}/reminder/{}",
        client.base_url(),
        athlete_id,
        gear_id,
        reminder_id
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
    async fn test_calc_gear_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/gear/g1/calc"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "distance": 5000.0,
                "time": 3600.0,
                "activities": 10
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let stats = calc_gear(&client, "a-001", "g1").await.unwrap();

        assert_eq!(stats.distance, Some(5000.0));
    }

    #[tokio::test]
    async fn test_create_gear_reminder_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/gear/g1/reminder"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "distance": 1000.0,
                "notes": "Replace chain"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let reminder = GearReminder {
            id: None,
            distance: Some(1000.0),
            date: None,
            notes: Some("Replace chain".to_string()),
            done: None,
        };
        let result = create_gear_reminder(&client, "a-001", "g1", &reminder)
            .await
            .unwrap();

        assert_eq!(result.notes.as_deref(), Some("Replace chain"));
    }

    #[tokio::test]
    async fn test_delete_gear_reminder_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/athlete/a-001/gear/g1/reminder/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_gear_reminder(&client, "a-001", "g1", 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_calc_gear_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/gear/g1/calc"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = calc_gear(&client, "a-001", "g1").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_gear_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/gear/g1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "g1",
                "name": "Road Bike",
                "distance": 5000.0
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let gear = get_gear(&client, "a-001", "g1").await.unwrap();

        assert_eq!(gear.id, "g1");
        assert_eq!(gear.name, Some("Road Bike".to_string()));
    }

    #[tokio::test]
    async fn test_create_gear_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/gear"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "g2",
                "name": "New Bike",
                "distance": 0.0
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateGearInput {
            name: Some("New Bike".to_string()),
            distance: Some(0.0),
            retired: Some(false),
        };
        let gear = create_gear(&client, "a-001", &input).await.unwrap();

        assert_eq!(gear.name, Some("New Bike".to_string()));
    }

    #[tokio::test]
    async fn test_update_gear_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/gear/g1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "g1",
                "name": "Updated Bike",
                "distance": 6000.0
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateGearInput {
            name: Some("Updated Bike".to_string()),
            distance: None,
            retired: None,
        };
        let gear = update_gear(&client, "a-001", "g1", &input).await.unwrap();

        assert_eq!(gear.name, Some("Updated Bike".to_string()));
    }

    #[tokio::test]
    async fn test_delete_gear_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/athlete/a-001/gear/g1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_gear(&client, "a-001", "g1").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_replace_gear_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/gear/g1/replace"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = ReplaceGearInput {
            new_gear_id: Some("g2".to_string()),
            retired_date: None,
        };
        let result = replace_gear(&client, "a-001", "g1", &input).await;

        assert!(result.is_ok());
    }
}
