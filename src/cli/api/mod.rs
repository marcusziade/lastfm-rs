// API client module

mod client;
mod endpoints;

pub use client::LastfmApiClient;
pub use endpoints::*;

use crate::cli::error::Result;
use std::collections::HashMap;

/// Build a cache key from method and parameters
pub fn build_cache_key(method: &str, params: &HashMap<String, String>) -> String {
    use crate::models::CacheKey;
    params.cache_key(method)
}

/// Validate required parameters for a method
pub fn validate_method_params(method: &str, params: &HashMap<String, String>) -> Result<()> {
    use crate::cli::error::CliError;

    match crate::common::validation::validate_method_params(method, params) {
        Ok(()) => Ok(()),
        Err(msg) => {
            // Try to extract specific parameter names for better error messages
            if msg.contains("Missing required parameter:") {
                let param = msg.split("parameter: ").nth(1).unwrap_or("unknown").trim();
                if param.contains(" or ") || param.contains(" and ") || param.contains(",") {
                    // Complex validation message
                    Err(CliError::validation(msg))
                } else {
                    // Single parameter
                    Err(CliError::missing_argument(param))
                }
            } else {
                Err(CliError::validation(msg))
            }
        }
    }
}
