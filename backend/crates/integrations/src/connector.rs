//! Connector Framework (Epic 150, Story 150.2)
//!
//! Standardized connector SDK with authentication, rate limiting, error handling,
//! and data transformation utilities.

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use reqwest::{header::HeaderMap, Client, Method, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::warn;

/// Connector errors.
#[derive(Debug, Error)]
pub enum ConnectorError {
    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Rate limit exceeded, retry after {0} seconds")]
    RateLimited(u64),

    #[error("Request timeout after {0}ms")]
    Timeout(u64),

    #[error("HTTP error: {status} - {message}")]
    HttpError { status: u16, message: String },

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Transform error: {0}")]
    TransformError(String),

    #[error("Retry exhausted after {0} attempts")]
    RetryExhausted(u32),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl From<reqwest::Error> for ConnectorError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            ConnectorError::Timeout(30000)
        } else if err.is_connect() {
            ConnectorError::NetworkError(err.to_string())
        } else {
            ConnectorError::NetworkError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for ConnectorError {
    fn from(err: serde_json::Error) -> Self {
        ConnectorError::SerializationError(err.to_string())
    }
}

/// Authentication configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthConfig {
    /// OAuth 2.0 authentication.
    OAuth2 {
        client_id: String,
        client_secret: String,
        token_url: String,
        scopes: Vec<String>,
        access_token: Option<String>,
        refresh_token: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    },
    /// API key authentication.
    ApiKey {
        key: String,
        header_name: Option<String>,
        query_param: Option<String>,
    },
    /// Basic authentication.
    Basic { username: String, password: String },
    /// Bearer token authentication.
    BearerToken { token: String },
    /// Custom authentication.
    Custom { headers: HashMap<String, String> },
    /// No authentication.
    None,
}

/// Rate limiter state.
#[derive(Debug, Clone)]
pub struct RateLimiterState {
    pub requests_made: u32,
    pub window_start: DateTime<Utc>,
    pub retry_after: Option<DateTime<Utc>>,
}

/// Rate limiter configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_window: u32,
    pub window_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_window: 100,
            window_seconds: 60,
        }
    }
}

/// Retry configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub exponential_backoff: bool,
    pub retry_on_status_codes: Vec<u16>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            exponential_backoff: true,
            retry_on_status_codes: vec![408, 429, 500, 502, 503, 504],
        }
    }
}

/// Connector configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorConfig {
    pub name: String,
    pub base_url: String,
    pub auth: AuthConfig,
    pub rate_limit: Option<RateLimitConfig>,
    pub retry: RetryConfig,
    pub timeout_ms: u64,
    pub default_headers: HashMap<String, String>,
}

impl ConnectorConfig {
    /// Create a new connector configuration.
    pub fn new(name: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            base_url: base_url.into(),
            auth: AuthConfig::None,
            rate_limit: None,
            retry: RetryConfig::default(),
            timeout_ms: 30000,
            default_headers: HashMap::new(),
        }
    }

    /// Set authentication.
    pub fn with_auth(mut self, auth: AuthConfig) -> Self {
        self.auth = auth;
        self
    }

    /// Set rate limiting.
    pub fn with_rate_limit(mut self, config: RateLimitConfig) -> Self {
        self.rate_limit = Some(config);
        self
    }

    /// Set retry configuration.
    pub fn with_retry(mut self, config: RetryConfig) -> Self {
        self.retry = config;
        self
    }

    /// Set timeout.
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Add a default header.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }
}

/// Request execution result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult<T> {
    pub data: Option<T>,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub duration_ms: u64,
    pub retry_count: u32,
    pub rate_limited: bool,
}

/// Connector execution log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLogEntry {
    pub connector_name: String,
    pub action: String,
    pub method: String,
    pub url: String,
    pub status_code: Option<u16>,
    pub error: Option<String>,
    pub duration_ms: u64,
    pub retry_count: u32,
    pub rate_limited: bool,
    pub timestamp: DateTime<Utc>,
}

/// HTTP connector client.
pub struct HttpConnector {
    config: ConnectorConfig,
    client: Client,
    rate_limiter: Arc<RwLock<RateLimiterState>>,
    log_callback: Option<Box<dyn Fn(ExecutionLogEntry) + Send + Sync>>,
}

impl HttpConnector {
    /// Create a new HTTP connector.
    pub fn new(config: ConnectorConfig) -> Result<Self, ConnectorError> {
        let mut client_builder =
            Client::builder().timeout(std::time::Duration::from_millis(config.timeout_ms));

        // Add default headers
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());

        for (key, value) in &config.default_headers {
            if let Ok(header_name) = reqwest::header::HeaderName::try_from(key.as_str()) {
                if let Ok(header_value) = reqwest::header::HeaderValue::from_str(value) {
                    headers.insert(header_name, header_value);
                }
            }
        }

        client_builder = client_builder.default_headers(headers);

        let client = client_builder
            .build()
            .map_err(|e| ConnectorError::ConfigurationError(e.to_string()))?;

        Ok(Self {
            config,
            client,
            rate_limiter: Arc::new(RwLock::new(RateLimiterState {
                requests_made: 0,
                window_start: Utc::now(),
                retry_after: None,
            })),
            log_callback: None,
        })
    }

    /// Set a logging callback.
    pub fn with_log_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(ExecutionLogEntry) + Send + Sync + 'static,
    {
        self.log_callback = Some(Box::new(callback));
        self
    }

    /// Check and update rate limiter.
    async fn check_rate_limit(&self) -> Result<(), ConnectorError> {
        let Some(ref rate_config) = self.config.rate_limit else {
            return Ok(());
        };

        let mut state = self.rate_limiter.write().await;

        // Check if we need to reset the window
        let window_duration = Duration::seconds(rate_config.window_seconds as i64);
        if Utc::now() - state.window_start > window_duration {
            state.requests_made = 0;
            state.window_start = Utc::now();
            state.retry_after = None;
        }

        // Check if we're still in retry-after period
        if let Some(retry_after) = state.retry_after {
            if Utc::now() < retry_after {
                let wait_seconds = (retry_after - Utc::now()).num_seconds() as u64;
                return Err(ConnectorError::RateLimited(wait_seconds));
            }
            state.retry_after = None;
        }

        // Check if we've exceeded the rate limit
        if state.requests_made >= rate_config.requests_per_window {
            let remaining = window_duration - (Utc::now() - state.window_start);
            return Err(ConnectorError::RateLimited(remaining.num_seconds() as u64));
        }

        state.requests_made += 1;
        Ok(())
    }

    /// Set retry-after from response.
    async fn set_retry_after(&self, seconds: u64) {
        let mut state = self.rate_limiter.write().await;
        state.retry_after = Some(Utc::now() + Duration::seconds(seconds as i64));
    }

    /// Apply authentication to request.
    fn apply_auth(&self, mut request: RequestBuilder) -> Result<RequestBuilder, ConnectorError> {
        match &self.config.auth {
            AuthConfig::OAuth2 { access_token, .. } => {
                if let Some(token) = access_token {
                    request = request.bearer_auth(token);
                } else {
                    return Err(ConnectorError::Authentication(
                        "No access token available".to_string(),
                    ));
                }
            }
            AuthConfig::ApiKey {
                key,
                header_name,
                query_param,
            } => {
                if let Some(header) = header_name {
                    request = request.header(header.as_str(), key.as_str());
                } else if let Some(param) = query_param {
                    request = request.query(&[(param.as_str(), key.as_str())]);
                } else {
                    request = request.header("X-API-Key", key.as_str());
                }
            }
            AuthConfig::Basic { username, password } => {
                request = request.basic_auth(username, Some(password));
            }
            AuthConfig::BearerToken { token } => {
                request = request.bearer_auth(token);
            }
            AuthConfig::Custom { headers } => {
                for (key, value) in headers {
                    request = request.header(key.as_str(), value.as_str());
                }
            }
            AuthConfig::None => {}
        }
        Ok(request)
    }

    /// Calculate retry delay.
    fn calculate_retry_delay(&self, attempt: u32) -> u64 {
        let base_delay = self.config.retry.initial_delay_ms;

        if self.config.retry.exponential_backoff {
            let delay = base_delay * 2u64.pow(attempt.saturating_sub(1));
            delay.min(self.config.retry.max_delay_ms)
        } else {
            base_delay
        }
    }

    /// Execute a request with retry logic.
    pub async fn execute<T, B>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
        extra_headers: Option<HashMap<String, String>>,
    ) -> Result<ExecutionResult<T>, ConnectorError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", self.config.base_url, path);
        let start_time = Utc::now();
        let mut last_error = None;
        let mut rate_limited = false;

        for attempt in 1..=self.config.retry.max_attempts {
            // Check rate limit
            if let Err(e) = self.check_rate_limit().await {
                if matches!(e, ConnectorError::RateLimited(_)) {
                    rate_limited = true;
                }
                last_error = Some(e);

                if attempt < self.config.retry.max_attempts {
                    let delay = self.calculate_retry_delay(attempt);
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                    continue;
                }
                break;
            }

            // Build request
            let mut request = self.client.request(method.clone(), &url);

            // Apply authentication
            request = self.apply_auth(request)?;

            // Add extra headers
            if let Some(headers) = &extra_headers {
                for (key, value) in headers {
                    request = request.header(key.as_str(), value.as_str());
                }
            }

            // Add body
            if let Some(b) = body {
                request = request.json(b);
            }

            // Execute request
            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => {
                    warn!(
                        "Request attempt {} failed for {}: {}",
                        attempt, self.config.name, e
                    );
                    last_error = Some(e.into());

                    if attempt < self.config.retry.max_attempts {
                        let delay = self.calculate_retry_delay(attempt);
                        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                    }
                    continue;
                }
            };

            let status = response.status();
            let status_code = status.as_u16();

            // Extract headers
            let response_headers: HashMap<String, String> = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();

            // Handle rate limit response
            if status_code == 429 {
                rate_limited = true;
                let retry_after = response_headers
                    .get("retry-after")
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(60);

                self.set_retry_after(retry_after).await;

                last_error = Some(ConnectorError::RateLimited(retry_after));

                if attempt < self.config.retry.max_attempts {
                    tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                }
                continue;
            }

            // Check if we should retry based on status code
            if self
                .config
                .retry
                .retry_on_status_codes
                .contains(&status_code)
            {
                last_error = Some(ConnectorError::HttpError {
                    status: status_code,
                    message: format!("HTTP {} error", status_code),
                });

                if attempt < self.config.retry.max_attempts {
                    let delay = self.calculate_retry_delay(attempt);
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                }
                continue;
            }

            let duration_ms = (Utc::now() - start_time).num_milliseconds() as u64;

            // Parse response body
            if status.is_success() {
                let body_text = response.text().await.unwrap_or_default();
                let data: Option<T> = if body_text.is_empty() {
                    None
                } else {
                    serde_json::from_str(&body_text).ok()
                };

                // Log execution
                if let Some(ref callback) = self.log_callback {
                    callback(ExecutionLogEntry {
                        connector_name: self.config.name.clone(),
                        action: path.to_string(),
                        method: method.to_string(),
                        url: url.clone(),
                        status_code: Some(status_code),
                        error: None,
                        duration_ms,
                        retry_count: attempt - 1,
                        rate_limited,
                        timestamp: Utc::now(),
                    });
                }

                return Ok(ExecutionResult {
                    data,
                    status_code,
                    headers: response_headers,
                    duration_ms,
                    retry_count: attempt - 1,
                    rate_limited,
                });
            } else {
                let error_body = response.text().await.unwrap_or_default();

                // Log execution
                if let Some(ref callback) = self.log_callback {
                    callback(ExecutionLogEntry {
                        connector_name: self.config.name.clone(),
                        action: path.to_string(),
                        method: method.to_string(),
                        url: url.clone(),
                        status_code: Some(status_code),
                        error: Some(error_body.clone()),
                        duration_ms,
                        retry_count: attempt - 1,
                        rate_limited,
                        timestamp: Utc::now(),
                    });
                }

                return Err(ConnectorError::HttpError {
                    status: status_code,
                    message: error_body,
                });
            }
        }

        let duration_ms = (Utc::now() - start_time).num_milliseconds() as u64;

        // Log failed execution
        if let Some(ref callback) = self.log_callback {
            callback(ExecutionLogEntry {
                connector_name: self.config.name.clone(),
                action: path.to_string(),
                method: method.to_string(),
                url: url.clone(),
                status_code: None,
                error: last_error.as_ref().map(|e| e.to_string()),
                duration_ms,
                retry_count: self.config.retry.max_attempts - 1,
                rate_limited,
                timestamp: Utc::now(),
            });
        }

        Err(last_error.unwrap_or(ConnectorError::RetryExhausted(
            self.config.retry.max_attempts,
        )))
    }

    /// Execute a GET request.
    pub async fn get<T>(&self, path: &str) -> Result<ExecutionResult<T>, ConnectorError>
    where
        T: DeserializeOwned,
    {
        self.execute::<T, ()>(Method::GET, path, None, None).await
    }

    /// Execute a POST request.
    pub async fn post<T, B>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<ExecutionResult<T>, ConnectorError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.execute(Method::POST, path, Some(body), None).await
    }

    /// Execute a PUT request.
    pub async fn put<T, B>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<ExecutionResult<T>, ConnectorError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.execute(Method::PUT, path, Some(body), None).await
    }

    /// Execute a PATCH request.
    pub async fn patch<T, B>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<ExecutionResult<T>, ConnectorError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.execute(Method::PATCH, path, Some(body), None).await
    }

    /// Execute a DELETE request.
    pub async fn delete<T>(&self, path: &str) -> Result<ExecutionResult<T>, ConnectorError>
    where
        T: DeserializeOwned,
    {
        self.execute::<T, ()>(Method::DELETE, path, None, None)
            .await
    }

    /// Update OAuth2 tokens.
    pub fn update_oauth_tokens(
        &mut self,
        access_token: String,
        refresh_token: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) {
        if let AuthConfig::OAuth2 {
            access_token: ref mut at,
            refresh_token: ref mut rt,
            expires_at: ref mut ea,
            ..
        } = self.config.auth
        {
            *at = Some(access_token);
            *rt = refresh_token;
            *ea = expires_at;
        }
    }

    /// Check if OAuth2 token needs refresh.
    pub fn needs_token_refresh(&self) -> bool {
        if let AuthConfig::OAuth2 { expires_at, .. } = &self.config.auth {
            if let Some(exp) = expires_at {
                // Refresh 5 minutes before expiry
                return Utc::now() + Duration::minutes(5) >= *exp;
            }
        }
        false
    }

    /// Get the connector name.
    pub fn name(&self) -> &str {
        &self.config.name
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }
}

/// Trait for connector actions.
#[async_trait]
pub trait ConnectorAction: Send + Sync {
    type Input: Serialize + Send;
    type Output: DeserializeOwned + Send;

    /// Get the action name.
    fn name(&self) -> &str;

    /// Get the HTTP method.
    fn method(&self) -> Method;

    /// Get the endpoint path, optionally with path parameters.
    fn path(&self, input: &Self::Input) -> String;

    /// Transform the input before sending.
    fn transform_input(&self, input: Self::Input) -> Result<serde_json::Value, ConnectorError> {
        serde_json::to_value(input).map_err(|e| ConnectorError::TransformError(e.to_string()))
    }

    /// Transform the output after receiving.
    fn transform_output(&self, output: serde_json::Value) -> Result<Self::Output, ConnectorError> {
        serde_json::from_value(output).map_err(|e| ConnectorError::TransformError(e.to_string()))
    }

    /// Execute the action.
    async fn execute(
        &self,
        connector: &HttpConnector,
        input: Self::Input,
    ) -> Result<ExecutionResult<Self::Output>, ConnectorError> {
        let path = self.path(&input);
        let body = self.transform_input(input)?;

        let result: ExecutionResult<serde_json::Value> = match self.method() {
            Method::GET => connector.get(&path).await?,
            Method::POST => connector.post(&path, &body).await?,
            Method::PUT => connector.put(&path, &body).await?,
            Method::PATCH => connector.patch(&path, &body).await?,
            Method::DELETE => connector.delete(&path).await?,
            _ => {
                return Err(ConnectorError::ConfigurationError(format!(
                    "Unsupported HTTP method: {}",
                    self.method()
                )))
            }
        };

        let output = if let Some(data) = result.data {
            Some(self.transform_output(data)?)
        } else {
            None
        };

        Ok(ExecutionResult {
            data: output,
            status_code: result.status_code,
            headers: result.headers,
            duration_ms: result.duration_ms,
            retry_count: result.retry_count,
            rate_limited: result.rate_limited,
        })
    }
}

/// Data transformer for mapping between systems.
pub struct DataTransformer {
    field_mappings: HashMap<String, String>,
    value_transformations:
        HashMap<String, Box<dyn Fn(serde_json::Value) -> serde_json::Value + Send + Sync>>,
}

impl DataTransformer {
    /// Create a new data transformer.
    pub fn new() -> Self {
        Self {
            field_mappings: HashMap::new(),
            value_transformations: HashMap::new(),
        }
    }

    /// Add a field mapping.
    pub fn map_field(mut self, source: impl Into<String>, target: impl Into<String>) -> Self {
        self.field_mappings.insert(source.into(), target.into());
        self
    }

    /// Add a value transformation.
    pub fn transform_value<F>(mut self, field: impl Into<String>, transformer: F) -> Self
    where
        F: Fn(serde_json::Value) -> serde_json::Value + Send + Sync + 'static,
    {
        self.value_transformations
            .insert(field.into(), Box::new(transformer));
        self
    }

    /// Transform a JSON object.
    pub fn transform(&self, input: serde_json::Value) -> Result<serde_json::Value, ConnectorError> {
        let Some(obj) = input.as_object() else {
            return Ok(input);
        };

        let mut output = serde_json::Map::new();

        for (key, value) in obj {
            // Apply field mapping
            let target_key = self.field_mappings.get(key).unwrap_or(key).clone();

            // Apply value transformation
            let transformed_value = if let Some(transformer) = self.value_transformations.get(key) {
                transformer(value.clone())
            } else {
                value.clone()
            };

            output.insert(target_key, transformed_value);
        }

        Ok(serde_json::Value::Object(output))
    }
}

impl Default for DataTransformer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connector_config_builder() {
        let config = ConnectorConfig::new("test", "https://api.example.com")
            .with_auth(AuthConfig::ApiKey {
                key: "test-key".to_string(),
                header_name: Some("X-API-Key".to_string()),
                query_param: None,
            })
            .with_timeout(5000)
            .with_header("User-Agent", "PPT/1.0");

        assert_eq!(config.name, "test");
        assert_eq!(config.base_url, "https://api.example.com");
        assert_eq!(config.timeout_ms, 5000);
        assert!(config.default_headers.contains_key("User-Agent"));
    }

    #[test]
    fn test_data_transformer() {
        let transformer = DataTransformer::new()
            .map_field("firstName", "first_name")
            .map_field("lastName", "last_name")
            .transform_value("email", |v| {
                if let Some(s) = v.as_str() {
                    serde_json::Value::String(s.to_lowercase())
                } else {
                    v
                }
            });

        let input = serde_json::json!({
            "firstName": "John",
            "lastName": "Doe",
            "email": "John.Doe@Example.Com"
        });

        let output = transformer.transform(input).unwrap();

        assert_eq!(output["first_name"], "John");
        assert_eq!(output["last_name"], "Doe");
        assert_eq!(output["email"], "john.doe@example.com");
    }

    #[test]
    fn test_retry_delay_calculation() {
        let config =
            ConnectorConfig::new("test", "https://api.example.com").with_retry(RetryConfig {
                max_attempts: 5,
                initial_delay_ms: 1000,
                max_delay_ms: 30000,
                exponential_backoff: true,
                retry_on_status_codes: vec![500],
            });

        let connector = HttpConnector::new(config).unwrap();

        assert_eq!(connector.calculate_retry_delay(1), 1000);
        assert_eq!(connector.calculate_retry_delay(2), 2000);
        assert_eq!(connector.calculate_retry_delay(3), 4000);
        assert_eq!(connector.calculate_retry_delay(4), 8000);
        assert_eq!(connector.calculate_retry_delay(5), 16000);
    }
}
