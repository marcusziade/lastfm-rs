use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Cache key generation
pub trait CacheKey {
    fn cache_key(&self, method: &str) -> String;
}

impl CacheKey for HashMap<String, String> {
    fn cache_key(&self, method: &str) -> String {
        let mut params: Vec<(String, String)> = self
            .iter()
            .filter(|(k, _)| {
                k.as_str() != "api_key" && k.as_str() != "format" && k.as_str() != "callback"
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        params.sort_by(|a, b| a.0.cmp(&b.0));

        let param_string = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");

        format!("lastfm:{method}:{param_string}")
    }
}

// Rate limit key
pub fn rate_limit_key(ip: &str) -> String {
    format!("rate_limit:{ip}")
}

// Method definitions from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodDefinition {
    pub description: String,
    pub http_method: String,
    pub parameters: Vec<ParameterDefinition>,
    pub requires_auth: bool,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub description: String,
    pub name: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub param_type: String,
    pub default: Option<serde_json::Value>,
    pub max: Option<u32>,
}

// Request signing
pub fn sign_request(params: &HashMap<String, String>, secret: &str) -> String {
    use crate::common::signing::{sign_request, HashAlgorithm};
    sign_request(params, secret, HashAlgorithm::SHA256)
}
