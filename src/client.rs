use reqwest::Client;
use std::time::Duration;
use thiserror::Error;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("INTERVALS_API_KEY environment variable not set")]
    MissingApiKey,

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },
}

pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .expect("failed to build HTTP client");
        Self {
            client,
            base_url,
            api_key,
        }
    }

    pub fn from_env(base_url: String) -> Result<Self, ApiError> {
        let api_key = std::env::var("INTERVALS_API_KEY").map_err(|_| ApiError::MissingApiKey)?;
        Ok(Self::new(base_url, api_key))
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_request_timeout_fires() {
        let mock_server = MockServer::start().await;

        // Respond with a 1-minute delay — much longer than REQUEST_TIMEOUT
        Mock::given(method("GET"))
            .and(path("/slow"))
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(60)))
            .mount(&mock_server)
            .await;

        let client = Client::builder()
            .timeout(Duration::from_millis(100))
            .build()
            .unwrap();

        let result = client
            .get(format!("{}/slow", mock_server.uri()))
            .send()
            .await;

        assert!(result.is_err(), "expected timeout error");
        let err = result.unwrap_err();
        assert!(
            err.is_timeout(),
            "expected is_timeout() == true, got: {err}"
        );
    }

    #[tokio::test]
    async fn test_api_client_new_builds_successfully() {
        // Verifies that Client::builder() with timeouts does not panic
        let _client = ApiClient::new("http://localhost".to_string(), "key".to_string());
    }
}
