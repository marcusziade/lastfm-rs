use std::collections::HashMap;
use worker::{Env, Request, Response};
use crate::error::{ApiError, ApiResult};
use crate::models::rate_limit_key;
use crate::utils::{get_client_ip, validate_signature};


// Add CORS headers to response
pub fn add_cors_headers(mut response: Response) -> Result<Response, worker::Error> {
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, X-Request-Signature")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    
    Ok(response)
}

// Rate limiting middleware
pub async fn rate_limit(req: &Request, env: &Env) -> ApiResult<()> {
    let ip = get_client_ip(req);
    let key = rate_limit_key(&ip);
    
    let kv = match env.kv("RATE_LIMIT") {
        Ok(kv) => kv,
        Err(e) => {
            worker::console_log!("Failed to get RATE_LIMIT KV: {:?}", e);
            return Err(ApiError::temporary_error());
        }
    };
    
    // Get current count
    worker::console_log!("Getting rate limit for key: {}", key);
    let count = match kv.get(&key).text().await {
        Ok(Some(count_str)) => {
            worker::console_log!("Current count: {}", count_str);
            count_str.parse::<u32>().unwrap_or(0)
        },
        Ok(None) => {
            worker::console_log!("No existing count");
            0
        },
        Err(e) => {
            worker::console_log!("Error getting count: {:?}", e);
            0
        }
    };
    
    // Check rate limit (100 requests per minute)
    if count >= 100 {
        return Err(ApiError::rate_limit_exceeded());
    }
    
    // Increment counter
    worker::console_log!("Incrementing counter to {}", count + 1);
    match kv.put(&key, (count + 1).to_string()) {
        Ok(builder) => {
            match builder.expiration_ttl(60).execute().await {
                Ok(_) => worker::console_log!("Rate limit updated successfully"),
                Err(e) => {
                    worker::console_log!("Failed to update rate limit: {:?}", e);
                    return Err(ApiError::temporary_error());
                }
            }
        }
        Err(e) => {
            worker::console_log!("Failed to create put builder: {:?}", e);
            return Err(ApiError::temporary_error());
        }
    }
    
    Ok(())
}

// Request validation middleware
pub async fn validate_request(
    req: &Request,
    env: &Env,
    params: &HashMap<String, String>,
    method_name: &str,
) -> ApiResult<()> {
    worker::console_log!("Starting validation for method: {}", method_name);
    
    // Validate signature if provided
    match validate_signature(req, env, params) {
        Ok(_) => worker::console_log!("Signature validation passed"),
        Err(e) => {
            worker::console_log!("Signature validation failed: {:?}", e);
            return Err(e);
        }
    }
    
    // Validate required parameters based on method
    match validate_method_params(method_name, params) {
        Ok(_) => worker::console_log!("Method params validation passed"),
        Err(e) => {
            worker::console_log!("Method params validation failed: {:?}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

// Validate parameters for specific methods
pub fn validate_method_params(method: &str, params: &HashMap<String, String>) -> ApiResult<()> {
    match crate::common::validation::validate_method_params(method, params) {
        Ok(()) => Ok(()),
        Err(msg) => Err(ApiError::invalid_parameters(msg)),
    }
}