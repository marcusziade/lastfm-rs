use std::collections::HashMap;
use worker::{Url, Error};

/// Build a Last.fm API URL with the given parameters
pub fn build_lastfm_url(
    base_url: &str,
    method: &str,
    params: &HashMap<String, String>,
    api_key: &str,
) -> Result<Url, Error> {
    let mut url = Url::parse(base_url)?;
    
    // Add method
    url.query_pairs_mut().append_pair("method", method);
    
    // Add API key
    url.query_pairs_mut().append_pair("api_key", api_key);
    
    // Add format
    url.query_pairs_mut().append_pair("format", "json");
    
    // Add other parameters
    for (key, value) in params {
        if key != "method" && key != "api_key" && key != "format" {
            url.query_pairs_mut().append_pair(key, value);
        }
    }
    
    Ok(url)
}