use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteEventsRangeInput {
    pub start_date_local: String,
    pub end_date_local: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateEventsRangeInput {
    pub start_date_local: String,
    pub end_date_local: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub event_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteEventsBulkInput {
    pub ids: Option<Vec<i64>>,
    pub external_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DuplicateEventsInput {
    #[serde(rename = "eventIds")]
    pub event_ids: Vec<i64>,
    #[serde(rename = "numCopies")]
    pub num_copies: i32,
    #[serde(rename = "weeksBetween", skip_serializing_if = "Option::is_none")]
    pub weeks_between: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplyPlanInput {
    pub folder_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_local: Option<String>,
}

pub async fn delete_events_range(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    start_date: &str,
    end_date: &str,
    categories: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let cats: String = categories
        .iter()
        .map(|c| format!("category={}", urlencoding::encode(c)))
        .collect::<Vec<_>>()
        .join("&");
    let url = format!(
        "{}/api/v1/athlete/{}/events?oldest={}&newest={}&{}",
        client.base_url(),
        athlete_id,
        urlencoding::encode(start_date),
        urlencoding::encode(end_date),
        cats
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

pub async fn update_events_range(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    start_date: &str,
    end_date: &str,
    categories: &[String],
    input: &UpdateEventsRangeInput,
) -> Result<(), Box<dyn std::error::Error>> {
    let cats: String = categories
        .iter()
        .map(|c| format!("category={}", urlencoding::encode(c)))
        .collect::<Vec<_>>()
        .join("&");
    let url = format!(
        "{}/api/v1/athlete/{}/events?oldest={}&newest={}&{}",
        client.base_url(),
        athlete_id,
        urlencoding::encode(start_date),
        urlencoding::encode(end_date),
        cats
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

pub async fn delete_events_bulk(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &DeleteEventsBulkInput,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/events/bulk-delete",
        client.base_url(),
        athlete_id
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

pub async fn create_events_bulk(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    events: &[super::get_delete_event::EventEx],
) -> Result<Vec<super::get_delete_event::Event>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/events/bulk",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .post(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(events)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let result = response
        .json::<Vec<super::get_delete_event::Event>>()
        .await?;
    Ok(result)
}

pub async fn duplicate_events(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &DuplicateEventsInput,
) -> Result<Vec<super::get_delete_event::Event>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/duplicate-events",
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

    let result = response
        .json::<Vec<super::get_delete_event::Event>>()
        .await?;
    Ok(result)
}

pub async fn apply_plan_to_events(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &ApplyPlanInput,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/events/apply-plan",
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

    Ok(())
}

pub async fn download_event_workout(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    event_id: i64,
    output_path: &str,
    ext: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/events/{}/download{}",
        client.base_url(),
        athlete_id,
        event_id,
        ext
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

    let bytes = response.bytes().await?;
    std::fs::write(output_path, &bytes)?;
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
    async fn test_delete_events_range_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/athlete/a-001/events"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_events_range(
            &client,
            "a-001",
            "2024-01-01",
            "2024-01-31",
            &["NOTE".to_string()],
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_events_range_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/events"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = UpdateEventsRangeInput {
            start_date_local: "2024-01-01".to_string(),
            end_date_local: "2024-01-31".to_string(),
            name: None,
            description: None,
            category: None,
            event_type: None,
        };
        let result = update_events_range(
            &client,
            "a-001",
            "2024-01-01",
            "2024-01-31",
            &["NOTE".to_string()],
            &input,
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_events_bulk_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/events/bulk-delete"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = DeleteEventsBulkInput {
            ids: Some(vec![1, 2]),
            external_ids: None,
        };
        let result = delete_events_bulk(&client, "a-001", &input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_events_bulk_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/events/bulk"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 1, "name": "Event 1"},
                {"id": 2, "name": "Event 2"}
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let events = vec![super::super::get_delete_event::EventEx {
            id: None,
            start_date_local: Some("2024-01-15".to_string()),
            event_type: Some("WORKOUT".to_string()),
            category: Some("WORKOUT".to_string()),
            name: Some("Event 1".to_string()),
            description: None,
            uid: None,
            notes: None,
            workout: None,
        }];
        let result = create_events_bulk(&client, "a-001", &events).await.unwrap();

        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_duplicate_events_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/duplicate-events"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": 3, "name": "Copy of Event"}
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = DuplicateEventsInput {
            event_ids: vec![1],
            num_copies: 1,
            weeks_between: Some(1),
        };
        let result = duplicate_events(&client, "a-001", &input).await.unwrap();

        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_apply_plan_to_events_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/events/apply-plan"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = ApplyPlanInput {
            folder_id: 1,
            start_date_local: None,
        };
        let result = apply_plan_to_events(&client, "a-001", &input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_download_event_workout_success() {
        let mock_server = MockServer::start().await;
        let output_path = std::env::temp_dir().join("test_event_workout.zwo");

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/events/1/download.zwo"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"<?xml version=\"1.0\"?>"))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result =
            download_event_workout(&client, "a-001", 1, output_path.to_str().unwrap(), ".zwo")
                .await;

        assert!(result.is_ok());
        assert!(output_path.exists());
        std::fs::remove_file(&output_path).ok();
    }
}
