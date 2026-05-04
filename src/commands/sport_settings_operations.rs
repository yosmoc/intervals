use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSportSettingInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lthr: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w_prime: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_max: Option<f64>,
}

pub async fn get_sport_setting(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    id: i64,
) -> Result<super::list_sport_settings::SportSettings, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/sport-settings/{}",
        client.base_url(),
        athlete_id,
        id
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

    let result = response
        .json::<super::list_sport_settings::SportSettings>()
        .await?;
    Ok(result)
}

pub async fn create_sport_setting(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    input: &CreateSportSettingInput,
) -> Result<super::list_sport_settings::SportSettings, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/sport-settings",
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
        .json::<super::list_sport_settings::SportSettings>()
        .await?;
    Ok(result)
}

pub async fn update_sport_settings(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    inputs: &[CreateSportSettingInput],
) -> Result<Vec<super::list_sport_settings::SportSettings>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/sport-settings",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(inputs)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let result = response
        .json::<Vec<super::list_sport_settings::SportSettings>>()
        .await?;
    Ok(result)
}

pub async fn update_sport_setting(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    id: i64,
    input: &CreateSportSettingInput,
) -> Result<super::list_sport_settings::SportSettings, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/sport-settings/{}",
        client.base_url(),
        athlete_id,
        id
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

    let result = response
        .json::<super::list_sport_settings::SportSettings>()
        .await?;
    Ok(result)
}

pub async fn delete_sport_setting(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/sport-settings/{}",
        client.base_url(),
        athlete_id,
        id
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

pub async fn apply_sport_setting(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/sport-settings/{}/apply",
        client.base_url(),
        athlete_id,
        id
    );
    let response = client
        .client()
        .put(&url)
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

pub async fn list_sport_setting_matching(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    id: i64,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/sport-settings/{}/matching-activities",
        client.base_url(),
        athlete_id,
        id
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

    let result = response.json::<Vec<serde_json::Value>>().await?;
    Ok(result)
}

pub async fn list_sport_setting_pace_distances(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    id: i64,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/sport-settings/{}/pace_distances",
        client.base_url(),
        athlete_id,
        id
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

    let wrapper: serde_json::Value = response.json().await?;
    let distances = wrapper
        .get("distances")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect())
        .unwrap_or_default();
    Ok(distances)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_sport_setting_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/sport-settings/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "name": "Cycling",
                "ftp": 250
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_sport_setting(&client, "a-001", 1).await.unwrap();

        assert_eq!(result.id, 1);
    }

    #[tokio::test]
    async fn test_create_sport_setting_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/athlete/a-001/sport-settings"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 2,
                "name": "Running",
                "lthr": 170
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateSportSettingInput {
            types: Some(vec!["Run".to_string()]),
            ftp: None,
            lthr: Some(170),
            name: Some("Running".to_string()),
            w_prime: None,
            p_max: None,
        };
        let result = create_sport_setting(&client, "a-001", &input)
            .await
            .unwrap();

        assert_eq!(result.name, Some("Running".to_string()));
    }

    #[tokio::test]
    async fn test_update_sport_setting_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/sport-settings/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "name": "Updated Cycling",
                "ftp": 260
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let input = CreateSportSettingInput {
            types: None,
            ftp: Some(260),
            lthr: None,
            name: Some("Updated Cycling".to_string()),
            w_prime: None,
            p_max: None,
        };
        let result = update_sport_setting(&client, "a-001", 1, &input)
            .await
            .unwrap();

        assert_eq!(result.ftp, Some(260));
    }

    #[tokio::test]
    async fn test_delete_sport_setting_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/athlete/a-001/sport-settings/1"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = delete_sport_setting(&client, "a-001", 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_apply_sport_setting_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/sport-settings/1/apply"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = apply_sport_setting(&client, "a-001", 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_sport_setting_matching_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/api/v1/athlete/a-001/sport-settings/1/matching-activities",
            ))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"id": "act-001"}
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = list_sport_setting_matching(&client, "a-001", 1)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn test_list_sport_setting_pace_distances_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/api/v1/athlete/a-001/sport-settings/1/pace_distances",
            ))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "distances": [1000.0, 5000.0, 10000.0]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = list_sport_setting_pace_distances(&client, "a-001", 1)
            .await
            .unwrap();

        assert_eq!(result.len(), 3);
    }
}
