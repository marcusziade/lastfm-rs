#[cfg(test)]
mod tests {
    use lastfm_proxy_worker::models::{sign_request, CacheKey};
    use std::collections::HashMap;

    #[test]
    fn test_cache_key_generation() {
        let mut params = HashMap::new();
        params.insert("artist".to_string(), "The Beatles".to_string());
        params.insert("api_key".to_string(), "test_key".to_string());
        params.insert("format".to_string(), "json".to_string());

        let cache_key = params.cache_key("artist.getInfo");

        // Should exclude api_key and format from cache key
        assert_eq!(cache_key, "lastfm:artist.getInfo:artist=The Beatles");
    }

    #[test]
    fn test_cache_key_sorting() {
        let mut params = HashMap::new();
        params.insert("track".to_string(), "Hey Jude".to_string());
        params.insert("artist".to_string(), "The Beatles".to_string());
        params.insert("autocorrect".to_string(), "1".to_string());

        let cache_key = params.cache_key("track.getInfo");

        // Parameters should be sorted alphabetically
        assert_eq!(
            cache_key,
            "lastfm:track.getInfo:artist=The Beatles&autocorrect=1&track=Hey Jude"
        );
    }

    #[test]
    fn test_request_signing() {
        let mut params = HashMap::new();
        params.insert("method".to_string(), "auth.getSession".to_string());
        params.insert("token".to_string(), "test_token".to_string());
        params.insert("api_key".to_string(), "test_api_key".to_string());

        let signature = sign_request(&params, "test_secret");

        // The signature should be deterministic
        assert!(!signature.is_empty());
        assert_eq!(signature.len(), 64); // SHA256 produces 64 hex characters
    }

    #[test]
    fn test_signature_excludes_certain_params() {
        let mut params1 = HashMap::new();
        params1.insert("artist".to_string(), "Radiohead".to_string());
        params1.insert("track".to_string(), "Creep".to_string());
        params1.insert("api_sig".to_string(), "should_be_ignored".to_string());
        params1.insert("format".to_string(), "json".to_string());
        params1.insert("callback".to_string(), "myCallback".to_string());

        let mut params2 = HashMap::new();
        params2.insert("artist".to_string(), "Radiohead".to_string());
        params2.insert("track".to_string(), "Creep".to_string());

        let sig1 = sign_request(&params1, "secret");
        let sig2 = sign_request(&params2, "secret");

        // Signatures should be the same as api_sig, format, and callback are excluded
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_rate_limit_key() {
        use lastfm_proxy_worker::models::rate_limit_key;

        let key = rate_limit_key("192.168.1.1");
        assert_eq!(key, "rate_limit:192.168.1.1");
    }
}
