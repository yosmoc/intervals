use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Route {
    pub id: String,
    pub name: String,
    pub distance: f64,
    pub elevation: f64,
    pub activity_count: i64,
}

pub async fn list_athlete_routes(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/routes",
        client.base_url(),
        athlete_id
    );
    let response = client
        .client()
        .get(&url)
        .header("Authorization", format!("Bearer {}", client.api_key()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {} - {}", status, body).into());
    }

    let routes = response.json::<Vec<Route>>().await?;
    Ok(routes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_athlete_routes_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/routes"))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "route-001",
                    "name": "Mountain Loop",
                    "distance": 50000.0,
                    "elevation": 800.0,
                    "activity_count": 12
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let routes = list_athlete_routes(&client, "12345").await.unwrap();

        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].name, "Mountain Loop");
    }

    #[tokio::test]
    async fn test_list_athlete_routes_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/routes"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let routes = list_athlete_routes(&client, "12345").await.unwrap();

        assert!(routes.is_empty());
    }

    #[tokio::test]
    async fn test_list_athlete_routes_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/routes"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_athlete_routes(&client, "12345").await;

        assert!(result.is_err());
    }
}
