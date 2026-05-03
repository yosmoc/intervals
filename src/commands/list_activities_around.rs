use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityStub {
    pub id: Option<String>,
    pub start_date_local: Option<String>,
    #[serde(rename = "type")]
    pub activity_type: Option<String>,
    pub name: Option<String>,
    pub distance: Option<f64>,
    pub elapsed_time: Option<i64>,
}

pub struct ListActivitiesAroundParams {
    pub activity_id: String,
    pub route_id: Option<i64>,
    pub limit: Option<i32>,
}

pub async fn list_activities_around(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    params: &ListActivitiesAroundParams,
) -> Result<Vec<ActivityStub>, Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/athlete/{}/activities-around?activity_id={}",
        client.base_url(),
        athlete_id,
        params.activity_id
    );
    if let Some(route_id) = params.route_id {
        url.push_str(&format!("&route_id={}", route_id));
    }
    if let Some(limit) = params.limit {
        url.push_str(&format!("&limit={}", limit));
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

    let activities = response.json::<Vec<ActivityStub>>().await?;
    Ok(activities)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_activities_around_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/activities-around"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "i1",
                    "start_date_local": "2024-01-14T10:00:00",
                    "type": "Run",
                    "name": "Previous Run",
                    "distance": 4000.0,
                    "elapsed_time": 1200
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let params = ListActivitiesAroundParams {
            activity_id: "i2".to_string(),
            route_id: None,
            limit: Some(5),
        };
        let activities = list_activities_around(&client, "a-001", &params)
            .await
            .unwrap();

        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0].name.as_deref(), Some("Previous Run"));
    }

    #[tokio::test]
    async fn test_list_activities_around_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/a-001/activities-around"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let params = ListActivitiesAroundParams {
            activity_id: "i2".to_string(),
            route_id: None,
            limit: None,
        };
        let result = list_activities_around(&client, "a-001", &params).await;

        assert!(result.is_err());
    }
}
