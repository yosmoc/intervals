use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Activity {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub activity_type: Option<String>,
    #[serde(default)]
    pub start_date_local: Option<String>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub elapsed_time: Option<i64>,
    #[serde(default)]
    pub moving_time: Option<i64>,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub total_elevation_gain: Option<f64>,
    #[serde(default)]
    pub average_speed: Option<f64>,
    #[serde(default)]
    pub max_speed: Option<f64>,
    #[serde(default)]
    pub average_heartrate: Option<f64>,
    #[serde(default)]
    pub max_heartrate: Option<f64>,
    #[serde(default)]
    pub calories: Option<f64>,
    #[serde(default)]
    pub device_name: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub external_id: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub icu_training_load: Option<i64>,
    #[serde(default)]
    pub icu_ftp: Option<i64>,
    #[serde(default)]
    pub icu_ctl: Option<f64>,
    #[serde(default)]
    pub icu_atl: Option<f64>,
    #[serde(default)]
    pub lthr: Option<i64>,
    #[serde(default)]
    pub athlete_max_hr: Option<i64>,
    #[serde(default)]
    pub commute: Option<bool>,
    #[serde(default)]
    pub trainer: Option<bool>,
    #[serde(default)]
    pub race: Option<bool>,
}

pub async fn get_activity(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    activity_id: &str,
) -> Result<Activity, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/activities/{}",
        client.base_url(),
        athlete_id,
        activity_id
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

    let activities = response.json::<Vec<Activity>>().await?;
    if activities.is_empty() {
        return Err("No activity returned".into());
    }
    Ok(activities.into_iter().next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_activity_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/activities/act-001"))
            .and(header("Authorization", "Basic QVBJX0tFWTp0ZXN0LWFwaS1rZXk="))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "act-001",
                    "start_date_local": "2024-01-15T08:00:00",
                    "type": "Ride",
                    "name": "Morning Ride",
                    "elapsed_time": 3600,
                    "distance": 25000.0,
                    "moving_time": 3500,
                    "total_elevation_gain": 200.0,
                    "average_speed": 7.0,
                    "max_speed": 15.0,
                    "average_heartrate": 150.0,
                    "max_heartrate": 175.0,
                    "calories": 800.0,
                    "device_name": "Garmin Edge 530",
                    "source": "GARMIN",
                    "icu_training_load": 85,
                    "icu_ctl": 60.0,
                    "icu_atl": 70.0,
                    "lthr": 170,
                    "athlete_max_hr": 185,
                    "commute": false,
                    "trainer": false,
                    "race": false
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let activity = get_activity(&client, "12345", "act-001").await.unwrap();

        assert_eq!(activity.id, "act-001");
        assert_eq!(activity.activity_type, Some("Ride".to_string()));
        assert_eq!(activity.name, Some("Morning Ride".to_string()));
        assert_eq!(activity.elapsed_time, Some(3600));
        assert_eq!(activity.distance, Some(25000.0));
    }

    #[tokio::test]
    async fn test_get_activity_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/activities/nonexistent"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({
                        "error": "Activity not found"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity(&client, "12345", "nonexistent").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_activity_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/athlete/12345/activities/act-001"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "error": "Unauthorized"
                    })),
            )
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_activity(&client, "12345", "act-001").await;

        assert!(result.is_err());
    }
}
