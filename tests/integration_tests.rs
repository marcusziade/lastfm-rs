#[cfg(test)]
mod integration_tests {
    use lastfm_proxy_worker::error::ApiError;
    use lastfm_proxy_worker::middleware::validate_method_params;
    use std::collections::HashMap;
    
    #[test]
    fn test_artist_get_correction_validation() {
        let mut params = HashMap::new();
        
        // Should fail without artist parameter
        let result = validate_method_params("artist.getCorrection", &params);
        assert!(result.is_err());
        
        // Should succeed with artist parameter
        params.insert("artist".to_string(), "The Beatles".to_string());
        let result = validate_method_params("artist.getCorrection", &params);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_artist_get_info_validation() {
        let mut params = HashMap::new();
        
        // Should fail without artist or mbid
        let result = validate_method_params("artist.getInfo", &params);
        assert!(result.is_err());
        
        // Should succeed with artist
        params.insert("artist".to_string(), "Radiohead".to_string());
        let result = validate_method_params("artist.getInfo", &params);
        assert!(result.is_ok());
        
        // Should also succeed with just mbid
        let mut params2 = HashMap::new();
        params2.insert("mbid".to_string(), "a74b1b7f-71a5-4011-9441-d0b5e4122711".to_string());
        let result = validate_method_params("artist.getInfo", &params2);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_album_validation() {
        let mut params = HashMap::new();
        
        // Album.getInfo should fail without (artist and album) or mbid
        let result = validate_method_params("album.getInfo", &params);
        assert!(result.is_err());
        
        // Should fail with just artist
        params.insert("artist".to_string(), "Pink Floyd".to_string());
        let result = validate_method_params("album.getInfo", &params);
        assert!(result.is_err());
        
        // Should succeed with artist and album
        params.insert("album".to_string(), "The Wall".to_string());
        let result = validate_method_params("album.getInfo", &params);
        assert!(result.is_ok());
        
        // Album.search should only need album
        let mut search_params = HashMap::new();
        let result = validate_method_params("album.search", &search_params);
        assert!(result.is_err());
        
        search_params.insert("album".to_string(), "Abbey Road".to_string());
        let result = validate_method_params("album.search", &search_params);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_track_validation() {
        let mut params = HashMap::new();
        
        // track.getCorrection needs both artist and track
        let result = validate_method_params("track.getCorrection", &params);
        assert!(result.is_err());
        
        params.insert("artist".to_string(), "Nirvana".to_string());
        let result = validate_method_params("track.getCorrection", &params);
        assert!(result.is_err());
        
        params.insert("track".to_string(), "Smells Like Teen Spirit".to_string());
        let result = validate_method_params("track.getCorrection", &params);
        assert!(result.is_ok());
        
        // track.search only needs track
        let mut search_params = HashMap::new();
        search_params.insert("track".to_string(), "Bohemian Rhapsody".to_string());
        let result = validate_method_params("track.search", &search_params);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_geo_validation() {
        let mut params = HashMap::new();
        
        // geo methods need country
        let result = validate_method_params("geo.getTopArtists", &params);
        assert!(result.is_err());
        
        params.insert("country".to_string(), "United Kingdom".to_string());
        let result = validate_method_params("geo.getTopArtists", &params);
        assert!(result.is_ok());
        
        let result = validate_method_params("geo.getTopTracks", &params);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_user_validation() {
        let mut params = HashMap::new();
        
        // Most user methods need user parameter
        let result = validate_method_params("user.getTopArtists", &params);
        assert!(result.is_err());
        
        params.insert("user".to_string(), "rj".to_string());
        let result = validate_method_params("user.getTopArtists", &params);
        assert!(result.is_ok());
        
        // user.getInfo doesn't require user parameter (optional)
        let empty_params = HashMap::new();
        let result = validate_method_params("user.getInfo", &empty_params);
        assert!(result.is_ok());
        
        // user.getPersonalTags needs user, tag, and taggingtype
        let mut personal_params = HashMap::new();
        personal_params.insert("user".to_string(), "test".to_string());
        let result = validate_method_params("user.getPersonalTags", &personal_params);
        assert!(result.is_err());
        
        personal_params.insert("tag".to_string(), "rock".to_string());
        let result = validate_method_params("user.getPersonalTags", &personal_params);
        assert!(result.is_err());
        
        personal_params.insert("taggingtype".to_string(), "artist".to_string());
        let result = validate_method_params("user.getPersonalTags", &personal_params);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_chart_validation() {
        // Chart methods don't require any parameters
        let params = HashMap::new();
        
        let result = validate_method_params("chart.getTopArtists", &params);
        assert!(result.is_ok());
        
        let result = validate_method_params("chart.getTopTags", &params);
        assert!(result.is_ok());
        
        let result = validate_method_params("chart.getTopTracks", &params);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_tag_validation() {
        let mut params = HashMap::new();
        
        // Most tag methods need tag parameter
        let result = validate_method_params("tag.getInfo", &params);
        assert!(result.is_err());
        
        params.insert("tag".to_string(), "electronic".to_string());
        let result = validate_method_params("tag.getInfo", &params);
        assert!(result.is_ok());
        
        // tag.getTopTags doesn't need any parameters
        let empty_params = HashMap::new();
        let result = validate_method_params("tag.getTopTags", &empty_params);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_unknown_method() {
        let params = HashMap::new();
        let result = validate_method_params("unknown.method", &params);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_api_error_codes() {
        let error = ApiError::invalid_parameters("Missing artist");
        assert_eq!(error.error, 6);
        
        let error = ApiError::invalid_api_key();
        assert_eq!(error.error, 10);
        
        let error = ApiError::service_offline();
        assert_eq!(error.error, 11);
        
        let error = ApiError::invalid_signature();
        assert_eq!(error.error, 13);
        
        let error = ApiError::temporary_error();
        assert_eq!(error.error, 16);
        
        let error = ApiError::rate_limit_exceeded();
        assert_eq!(error.error, 29);
    }
}