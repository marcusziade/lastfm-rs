// API client implementation

use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::any::Any;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use url::Url;

use crate::cli::{
    error::{CliError, Result},
    traits::{ApiClient, CacheManager},
};

use super::{build_cache_key, validate_method_params};

/// Last.fm API client implementation
pub struct LastfmApiClient {
    http_client: Client,
    base_url: String,
    api_key: Option<String>,
    cache: Option<Box<dyn CacheManager>>,
    timeout: Duration,
}

impl Clone for LastfmApiClient {
    fn clone(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone(),
            cache: None, // Don't clone cache
            timeout: self.timeout,
        }
    }
}

impl LastfmApiClient {
    /// Create a new API client
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("lastfm-cli/1.0")
            .build()
            .unwrap();

        Self {
            http_client,
            base_url,
            api_key,
            cache: None,
            timeout: Duration::from_secs(30),
        }
    }

    /// Set the cache manager
    pub fn with_cache(mut self, cache: Box<dyn CacheManager>) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Set the request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Get the API key
    pub fn get_api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Make a raw request with custom parameters
    pub async fn request_raw(&self, params: &HashMap<String, String>) -> Result<String> {
        let mut url = Url::parse(&self.base_url)
            .map_err(|e| CliError::other(format!("Invalid base URL: {e}")))?;

        // Add all parameters to URL
        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }

        // Add format if not present
        if !params.contains_key("format") {
            url.query_pairs_mut().append_pair("format", "json");
        }

        let response = self.http_client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(CliError::api(format!("HTTP {}", response.status())));
        }

        response.text().await.map_err(Into::into)
    }

    /// Build the full URL for a request
    fn build_url(&self, endpoint: &str, params: &HashMap<String, String>) -> Result<Url> {
        let mut url = Url::parse(&format!("{}{}", self.base_url, endpoint))
            .map_err(|e| CliError::other(format!("Invalid URL: {e}")))?;

        // Add query parameters
        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }

        Ok(url)
    }

    /// Extract method name from endpoint
    fn extract_method(&self, endpoint: &str) -> String {
        endpoint.trim_start_matches('/').replace('/', ".")
    }

    /// Check if response is an error
    fn check_response_error(&self, response: &Value) -> Result<()> {
        if let Some(error) = response.get("error") {
            let error_code = error.as_u64().unwrap_or(0);
            let message = response
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");

            match error_code {
                6 => Err(CliError::validation(message)),
                10 => Err(CliError::api("Invalid API key")),
                11 => Err(CliError::api("Service offline")),
                13 => Err(CliError::api("Invalid signature")),
                16 => Err(CliError::api("Temporary error")),
                29 => Err(CliError::RateLimit),
                _ => Err(CliError::api(message)),
            }
        } else {
            Ok(())
        }
    }
}

#[async_trait]
impl ApiClient for LastfmApiClient {
    async fn get(&self, endpoint: &str, params: &HashMap<String, String>) -> Result<Value> {
        let method = self.extract_method(endpoint);

        // Validate parameters
        validate_method_params(&method, params)?;

        // Check cache if available
        let cache_key = build_cache_key(&method, params);
        if let Some(cache) = &self.cache {
            if let Ok(Some(cached)) = cache.get(&cache_key).await {
                if let Ok(value) = serde_json::from_str::<Value>(&cached) {
                    return Ok(value);
                }
            }
        }

        // Build request URL
        let url = self.build_url(endpoint, params)?;

        // Make request
        let start = Instant::now();
        let response = self
            .http_client
            .get(url)
            .timeout(self.timeout)
            .send()
            .await?;

        let _elapsed = start.elapsed();

        // Check HTTP status
        if !response.status().is_success() {
            return Err(CliError::api(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        // Parse response
        let response_text = response.text().await?;
        let response_value: Value = serde_json::from_str(&response_text)?;

        // Check for API errors
        self.check_response_error(&response_value)?;

        // Cache successful response
        if let Some(cache) = &self.cache {
            let cache_ttl = 3600; // 1 hour
            let _ = cache.set(&cache_key, &response_text, cache_ttl).await;
        }

        Ok(response_value)
    }

    async fn post(&self, endpoint: &str, body: &Value) -> Result<Value> {
        let url = self.build_url(endpoint, &HashMap::new())?;

        let response = self
            .http_client
            .post(url)
            .json(body)
            .timeout(self.timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(CliError::api(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let response_value: Value = response.json().await?;
        self.check_response_error(&response_value)?;

        Ok(response_value)
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn health_check(&self) -> Result<bool> {
        let mut params = HashMap::new();
        params.insert("limit".to_string(), "1".to_string());

        match self.get("/chart/getTopArtists", &params).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_method() {
        let client = LastfmApiClient::new("http://example.com".to_string(), None);

        assert_eq!(client.extract_method("/artist/getInfo"), "artist.getInfo");
        assert_eq!(client.extract_method("/album/search"), "album.search");
        assert_eq!(
            client.extract_method("track/getSimilar"),
            "track.getSimilar"
        );
    }

    #[test]
    fn test_build_cache_key() {
        let mut params = HashMap::new();
        params.insert("artist".to_string(), "The Beatles".to_string());
        params.insert("api_key".to_string(), "test_key".to_string());
        params.insert("format".to_string(), "json".to_string());

        let cache_key = build_cache_key("artist.getInfo", &params);

        // Should exclude api_key and format
        assert_eq!(cache_key, "lastfm:artist.getInfo:artist=The Beatles");
    }
}
