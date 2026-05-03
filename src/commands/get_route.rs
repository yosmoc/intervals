use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AthleteRoute {
    pub athlete_id: Option<String>,
    pub route_id: Option<i64>,
    pub name: Option<String>,
    pub rename_activities: Option<bool>,
    pub commute: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
    pub replaced_by_route_id: Option<i64>,
    pub latlngs: Option<Vec<Vec<f64>>>,
}

pub async fn get_route(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    route_id: i64,
    include_path: bool,
) -> Result<AthleteRoute, Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/athlete/{}/routes/{}",
        client.base_url(),
        athlete_id,
        route_id
    );
    if include_path {
        url.push_str("?includePath=true");
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

    let route = response.json::<AthleteRoute>().await?;
    Ok(route)
}

pub async fn update_route(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    route_id: i64,
    route: &AthleteRoute,
) -> Result<AthleteRoute, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/routes/{}",
        client.base_url(),
        athlete_id,
        route_id
    );
    let response = client
        .client()
        .put(&url)
        .basic_auth("API_KEY", Some(client.api_key()))
        .json(route)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let route = response.json::<AthleteRoute>().await?;
    Ok(route)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_route_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/routes/123"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "athlete_id": "a-001",
                "route_id": 123,
                "name": "Morning Loop",
                "description": "A nice morning ride",
                "tags": ["hilly", "scenic"]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let route = get_route(&client, "a-001", 123, false).await.unwrap();

        assert_eq!(route.route_id, Some(123));
        assert_eq!(route.name.as_deref(), Some("Morning Loop"));
    }

    #[tokio::test]
    async fn test_get_route_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/routes/123"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_route(&client, "a-001", 123, false).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_route_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/athlete/a-001/routes/123"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "athlete_id": "a-001",
                "route_id": 123,
                "name": "Updated Route",
                "description": "Updated description"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let route = AthleteRoute {
            athlete_id: None,
            route_id: Some(123),
            name: Some("Updated Route".to_string()),
            rename_activities: None,
            commute: None,
            tags: None,
            description: Some("Updated description".to_string()),
            replaced_by_route_id: None,
            latlngs: None,
        };
        let result = update_route(&client, "a-001", 123, &route).await;

        assert!(result.is_ok());
    }
}
