use serde::{Deserialize, Serialize};

// HR Curve
#[derive(Debug, Deserialize, Serialize)]
pub struct HRCurvePoint {
    pub hr: f64,
    #[serde(default)]
    pub time: Option<f64>,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HRCurve {
    #[serde(default)]
    pub points: Vec<HRCurvePoint>,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

// HR Histogram
#[derive(Debug, Deserialize, Serialize)]
pub struct HRHistogramBin {
    #[serde(default)]
    pub hr: Option<f64>,
    #[serde(default)]
    pub time: Option<f64>,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HRHistogram {
    #[serde(default)]
    pub bins: Vec<HRHistogramBin>,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

// HR Load Model
#[derive(Debug, Deserialize, Serialize)]
pub struct HRLoadModel {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

// Pace Curve
#[derive(Debug, Deserialize, Serialize)]
pub struct PaceCurvePoint {
    #[serde(default)]
    pub pace: Option<f64>,
    #[serde(default)]
    pub time: Option<f64>,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaceCurve {
    #[serde(default)]
    pub points: Vec<PaceCurvePoint>,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

// Pace Histogram
#[derive(Debug, Deserialize, Serialize)]
pub struct PaceHistogramBin {
    #[serde(default)]
    pub pace: Option<f64>,
    #[serde(default)]
    pub time: Option<f64>,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaceHistogram {
    #[serde(default)]
    pub bins: Vec<PaceHistogramBin>,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

// Power Curves
#[derive(Debug, Deserialize, Serialize)]
pub struct PowerCurvePoint {
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub power: Option<f64>,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerCurves {
    #[serde(default)]
    pub points: Vec<PowerCurvePoint>,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

// Power Histogram
#[derive(Debug, Deserialize, Serialize)]
pub struct PowerHistogramBin {
    #[serde(default)]
    pub power: Option<f64>,
    #[serde(default)]
    pub time: Option<f64>,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerHistogram {
    #[serde(default)]
    pub bins: Vec<PowerHistogramBin>,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

// Power Spike Model
#[derive(Debug, Deserialize, Serialize)]
pub struct PowerSpikeModel {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

// Power vs HR
#[derive(Debug, Deserialize, Serialize)]
pub struct PowerVsHRPoint {
    #[serde(default)]
    pub power: Option<f64>,
    #[serde(default)]
    pub hr: Option<f64>,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerVsHR {
    #[serde(default)]
    pub points: Vec<PowerVsHRPoint>,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

// GAP Histogram
#[derive(Debug, Deserialize, Serialize)]
pub struct GAPHistogramBin {
    #[serde(default)]
    pub gap: Option<f64>,
    #[serde(default)]
    pub time: Option<f64>,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GAPHistogram {
    #[serde(default)]
    pub bins: Vec<GAPHistogramBin>,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

// Time at HR
#[derive(Debug, Deserialize, Serialize)]
pub struct TimeAtHRPoint {
    #[serde(default)]
    pub hr: Option<f64>,
    #[serde(default)]
    pub time: Option<f64>,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeAtHR {
    #[serde(default)]
    pub points: Vec<TimeAtHRPoint>,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

pub async fn get_activity_hr_curve(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<HRCurve, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/hr-curve",
        client.base_url(),
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

    let result = response.json::<HRCurve>().await?;
    Ok(result)
}

pub async fn get_activity_hr_histogram(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<HRHistogram, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/hr-histogram",
        client.base_url(),
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

    let result = response.json::<HRHistogram>().await?;
    Ok(result)
}

pub async fn get_activity_hr_load_model(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<HRLoadModel, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/hr-load-model",
        client.base_url(),
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

    let result = response.json::<HRLoadModel>().await?;
    Ok(result)
}

pub async fn get_activity_pace_curve(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<PaceCurve, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/pace-curve",
        client.base_url(),
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

    let result = response.json::<PaceCurve>().await?;
    Ok(result)
}

pub async fn get_activity_pace_histogram(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<PaceHistogram, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/pace-histogram",
        client.base_url(),
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

    let result = response.json::<PaceHistogram>().await?;
    Ok(result)
}

pub async fn get_activity_power_curves(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<PowerCurves, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/power-curves",
        client.base_url(),
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

    let result = response.json::<PowerCurves>().await?;
    Ok(result)
}

pub async fn get_activity_power_curve(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<PowerCurves, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/power-curve",
        client.base_url(),
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

    let result = response.json::<PowerCurves>().await?;
    Ok(result)
}

pub async fn get_activity_power_histogram(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<PowerHistogram, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/power-histogram",
        client.base_url(),
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

    let result = response.json::<PowerHistogram>().await?;
    Ok(result)
}

pub async fn get_activity_power_spike_model(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<PowerSpikeModel, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/power-spike-model",
        client.base_url(),
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

    let result = response.json::<PowerSpikeModel>().await?;
    Ok(result)
}

pub async fn get_activity_power_vs_hr(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<PowerVsHR, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/power-vs-hr",
        client.base_url(),
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

    let result = response.json::<PowerVsHR>().await?;
    Ok(result)
}

pub async fn get_activity_gap_histogram(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<GAPHistogram, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/gap-histogram",
        client.base_url(),
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

    let result = response.json::<GAPHistogram>().await?;
    Ok(result)
}

pub async fn get_activity_time_at_hr(
    client: &crate::client::ApiClient,
    activity_id: &str,
) -> Result<TimeAtHR, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/activity/{}/time-at-hr",
        client.base_url(),
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

    let result = response.json::<TimeAtHR>().await?;
    Ok(result)
}

// Best HR Curves
#[derive(Debug, Deserialize, Serialize)]
pub struct BestHRCurve {
    pub id: String,
    pub label: String,
    pub days: i64,
    #[serde(default)]
    pub secs: Option<Vec<f64>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BestHRCurveResponse {
    #[serde(default)]
    pub list: Vec<BestHRCurve>,
}

pub async fn list_athlete_hr_curves_best(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<BestHRCurve>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/hr-curves-best",
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

    let wrapper = response.json::<BestHRCurveResponse>().await?;
    Ok(wrapper.list)
}

// Best Power Curves
#[derive(Debug, Deserialize, Serialize)]
pub struct BestPowerCurve {
    pub id: String,
    pub label: String,
    pub days: i64,
    #[serde(default)]
    pub secs: Option<Vec<f64>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BestPowerCurveResponse {
    #[serde(default)]
    pub list: Vec<BestPowerCurve>,
}

pub async fn list_athlete_power_curves_best(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<BestPowerCurve>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/power-curves-best",
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

    let wrapper = response.json::<BestPowerCurveResponse>().await?;
    Ok(wrapper.list)
}

// Best Pace Curves
#[derive(Debug, Deserialize, Serialize)]
pub struct BestPaceCurve {
    pub id: String,
    pub label: String,
    pub days: i64,
    #[serde(default)]
    pub secs: Option<Vec<f64>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BestPaceCurveResponse {
    #[serde(default)]
    pub list: Vec<BestPaceCurve>,
}

pub async fn list_athlete_pace_curves_best(
    client: &crate::client::ApiClient,
    athlete_id: &str,
) -> Result<Vec<BestPaceCurve>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/v1/athlete/{}/pace-curves-best",
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

    let wrapper = response.json::<BestPaceCurveResponse>().await?;
    Ok(wrapper.list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ApiClient;
    use crate::commands::TEST_AUTH_HEADER;

    use wiremock::matchers::{header, method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_activity_hr_curve_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/hr-curve"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "points": [
                    {"hr": 150.0, "time": 60.0}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_hr_curve(&client, "act-001").await.unwrap();

        assert_eq!(result.points.len(), 1);
        assert_eq!(result.points[0].hr, 150.0);
    }

    #[tokio::test]
    async fn test_get_activity_hr_curve_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/hr-curve"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Unauthorized"
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "wrong-key".to_string());
        let result = get_activity_hr_curve(&client, "act-001").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_activity_hr_histogram_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/hr-histogram"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "bins": [
                    {"hr": 150.0, "time": 300.0}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_hr_histogram(&client, "act-001").await.unwrap();

        assert_eq!(result.bins.len(), 1);
        assert_eq!(result.bins[0].hr, Some(150.0));
    }

    #[tokio::test]
    async fn test_get_activity_hr_load_model_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/hr-load-model"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "model": "trimp",
                "data": [1.0, 2.0, 3.0]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_hr_load_model(&client, "act-001")
            .await
            .unwrap();

        assert!(result.data.get("model").is_some());
    }

    #[tokio::test]
    async fn test_get_activity_pace_curve_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/pace-curve"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "points": [
                    {"pace": 300.0, "time": 60.0}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_pace_curve(&client, "act-001").await.unwrap();

        assert_eq!(result.points.len(), 1);
        assert_eq!(result.points[0].pace, Some(300.0));
    }

    #[tokio::test]
    async fn test_get_activity_pace_histogram_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/pace-histogram"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "bins": [
                    {"pace": 300.0, "time": 300.0}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_pace_histogram(&client, "act-001")
            .await
            .unwrap();

        assert_eq!(result.bins.len(), 1);
    }

    #[tokio::test]
    async fn test_get_activity_power_curves_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/power-curves"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "points": [
                    {"duration": 60.0, "power": 350.0}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_power_curves(&client, "act-001").await.unwrap();

        assert_eq!(result.points.len(), 1);
        assert_eq!(result.points[0].power, Some(350.0));
    }

    #[tokio::test]
    async fn test_get_activity_power_curve_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/power-curve"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "points": [
                    {"duration": 60.0, "power": 350.0}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_power_curve(&client, "act-001").await.unwrap();

        assert_eq!(result.points.len(), 1);
    }

    #[tokio::test]
    async fn test_get_activity_power_histogram_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/power-histogram"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "bins": [
                    {"power": 350.0, "time": 60.0}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_power_histogram(&client, "act-001")
            .await
            .unwrap();

        assert_eq!(result.bins.len(), 1);
    }

    #[tokio::test]
    async fn test_get_activity_power_spike_model_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/power-spike-model"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "threshold": 400.0,
                "duration": 5
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_power_spike_model(&client, "act-001")
            .await
            .unwrap();

        assert!(result.data.get("threshold").is_some());
    }

    #[tokio::test]
    async fn test_get_activity_power_vs_hr_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/power-vs-hr"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "points": [
                    {"power": 300.0, "hr": 160.0}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_power_vs_hr(&client, "act-001").await.unwrap();

        assert_eq!(result.points.len(), 1);
    }

    #[tokio::test]
    async fn test_get_activity_gap_histogram_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/gap-histogram"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "bins": [
                    {"gap": 300.0, "time": 120.0}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_gap_histogram(&client, "act-001")
            .await
            .unwrap();

        assert_eq!(result.bins.len(), 1);
    }

    #[tokio::test]
    async fn test_get_activity_time_at_hr_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/activity/act-001/time-at-hr"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "points": [
                    {"hr": 150.0, "time": 300.0}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let result = get_activity_time_at_hr(&client, "act-001").await.unwrap();

        assert_eq!(result.points.len(), 1);
    }

    #[tokio::test]
    async fn test_list_athlete_hr_curves_best_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/hr-curves-best"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "list": [
                    {"id": "1y", "label": "1 year", "days": 365, "secs": [1, 2, 3]}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let curves = list_athlete_hr_curves_best(&client, "12345").await.unwrap();

        assert_eq!(curves.len(), 1);
        assert_eq!(curves[0].id, "1y");
    }

    #[tokio::test]
    async fn test_list_athlete_power_curves_best_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/power-curves-best"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "list": [
                    {"id": "all", "label": "All time", "days": 3650, "secs": [1, 2, 3]}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let curves = list_athlete_power_curves_best(&client, "12345")
            .await
            .unwrap();

        assert_eq!(curves.len(), 1);
        assert_eq!(curves[0].id, "all");
    }

    #[tokio::test]
    async fn test_list_athlete_pace_curves_best_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex("/api/v1/athlete/.*/pace-curves-best"))
            .and(header("Authorization", TEST_AUTH_HEADER))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "list": [
                    {"id": "1y", "label": "1 year", "days": 365, "secs": [1, 2, 3]}
                ]
            })))
            .mount(&mock_server)
            .await;

        let client = ApiClient::new(mock_server.uri(), "test-api-key".to_string());
        let curves = list_athlete_pace_curves_best(&client, "12345")
            .await
            .unwrap();

        assert_eq!(curves.len(), 1);
        assert_eq!(curves[0].label, "1 year");
    }
}
