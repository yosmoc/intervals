use serde::Deserialize;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct WellnessRecord {
    pub id: String,
    #[serde(default)]
    pub ctl: Option<f32>,
    #[serde(default)]
    pub atl: Option<f32>,
    #[serde(default)]
    pub ramp_rate: Option<f32>,
    #[serde(default)]
    pub weight: Option<f32>,
    #[serde(default)]
    #[serde(rename = "restingHR")]
    pub resting_hr: Option<i32>,
    #[serde(default)]
    pub hrv: Option<f32>,
}

pub struct ListWellnessParams {
    pub oldest: Option<String>,
    pub newest: Option<String>,
}

pub async fn list_wellness(
    client: &crate::client::ApiClient,
    athlete_id: &str,
    params: &ListWellnessParams,
) -> Result<Vec<WellnessRecord>, Box<dyn std::error::Error>> {
    let mut url = format!(
        "{}/api/v1/athlete/{}/wellness",
        client.base_url(),
        athlete_id
    );
    let mut query_params = Vec::new();

    if let Some(ref oldest) = params.oldest {
        query_params.push(format!("oldest={}", oldest));
    }
    if let Some(ref newest) = params.newest {
        query_params.push(format!("newest={}", newest));
    }

    if !query_params.is_empty() {
        url.push('?');
        url.push_str(&query_params.join("&"));
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

    let records = response.json::<Vec<WellnessRecord>>().await?;
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_wellness_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/wellness"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "2024-01-15",
                    "ctl": 85.5,
                    "atl": 90.2,
                    "rampRate": 4.7,
                    "weight": 72.5,
                    "restingHR": 48,
                    "hrv": 65.3
                },
                {
                    "id": "2024-01-16",
                    "ctl": 86.0,
                    "atl": 88.1,
                    "rampRate": 2.1,
                    "weight": 72.3,
                    "restingHR": 47,
                    "hrv": 68.1
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let records = list_wellness(
            &client,
            "12345",
            &ListWellnessParams {
                oldest: None,
                newest: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].id, "2024-01-15");
        assert_eq!(records[0].ctl, Some(85.5));
        assert_eq!(records[0].resting_hr, Some(48));
        assert_eq!(records[1].id, "2024-01-16");
    }

    #[tokio::test]
    async fn test_list_wellness_with_date_range() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/wellness"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let records = list_wellness(
            &client,
            "12345",
            &ListWellnessParams {
                oldest: Some("2024-01-01".to_string()),
                newest: Some("2024-01-31".to_string()),
            },
        )
        .await
        .unwrap();

        assert!(records.is_empty());
    }

    #[tokio::test]
    async fn test_list_wellness_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/wellness"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = list_wellness(
            &client,
            "12345",
            &ListWellnessParams {
                oldest: None,
                newest: None,
            },
        )
        .await;

        assert!(result.is_err());
    }
}
