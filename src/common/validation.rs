use std::collections::HashMap;

/// Validates parameters for Last.fm API methods
/// Returns Ok(()) if valid, Err(String) with error message if invalid
pub fn validate_method_params(method: &str, params: &HashMap<String, String>) -> Result<(), String> {
    match method {
        // Artist methods
        "artist.getCorrection" => {
            if !params.contains_key("artist") {
                return Err("Missing required parameter: artist".to_string());
            }
        }
        "artist.getInfo" | "artist.getSimilar" | "artist.getTopAlbums" | 
        "artist.getTopTags" | "artist.getTopTracks" => {
            if !params.contains_key("artist") && !params.contains_key("mbid") {
                return Err("Missing required parameter: artist or mbid".to_string());
            }
        }
        "artist.search" => {
            if !params.contains_key("artist") {
                return Err("Missing required parameter: artist".to_string());
            }
        }
        
        // Album methods
        "album.getInfo" | "album.getTopTags" => {
            if (!params.contains_key("artist") || !params.contains_key("album")) && !params.contains_key("mbid") {
                return Err("Missing required parameters: (artist and album) or mbid".to_string());
            }
        }
        "album.search" => {
            if !params.contains_key("album") {
                return Err("Missing required parameter: album".to_string());
            }
        }
        
        // Track methods
        "track.getCorrection" => {
            if !params.contains_key("artist") || !params.contains_key("track") {
                return Err("Missing required parameters: artist and track".to_string());
            }
        }
        "track.getInfo" | "track.getSimilar" => {
            if (!params.contains_key("artist") || !params.contains_key("track")) && !params.contains_key("mbid") {
                return Err("Missing required parameters: (artist and track) or mbid".to_string());
            }
        }
        "track.getTopTags" => {
            if !params.contains_key("artist") || !params.contains_key("track") {
                return Err("Missing required parameters: artist and track".to_string());
            }
        }
        "track.search" => {
            if !params.contains_key("track") {
                return Err("Missing required parameter: track".to_string());
            }
        }
        
        // Chart methods - no required parameters
        "chart.getTopArtists" | "chart.getTopTags" | "chart.getTopTracks" => {}
        
        // Geo methods
        "geo.getTopArtists" | "geo.getTopTracks" => {
            if !params.contains_key("country") {
                return Err("Missing required parameter: country".to_string());
            }
        }
        
        // Tag methods
        "tag.getInfo" | "tag.getSimilar" | "tag.getTopAlbums" | 
        "tag.getTopArtists" | "tag.getTopTracks" | "tag.getWeeklyChartList" => {
            if !params.contains_key("tag") {
                return Err("Missing required parameter: tag".to_string());
            }
        }
        "tag.getTopTags" => {} // No required parameters
        
        // User methods
        "user.getFriends" | "user.getLovedTracks" | "user.getRecentTracks" |
        "user.getTopAlbums" | "user.getTopArtists" | "user.getTopTags" |
        "user.getTopTracks" | "user.getWeeklyAlbumChart" | "user.getWeeklyArtistChart" |
        "user.getWeeklyChartList" | "user.getWeeklyTrackChart" => {
            if !params.contains_key("user") {
                return Err("Missing required parameter: user".to_string());
            }
        }
        "user.getPersonalTags" => {
            if !params.contains_key("user") || !params.contains_key("tag") || !params.contains_key("taggingtype") {
                return Err("Missing required parameters: user, tag, and taggingtype".to_string());
            }
        }
        "user.getInfo" => {} // User parameter is optional (defaults to authenticated user)
        
        // Library methods
        "library.getArtists" => {
            if !params.contains_key("user") {
                return Err("Missing required parameter: user".to_string());
            }
        }
        
        // Auth methods
        "auth.getSession" => {
            if !params.contains_key("token") {
                return Err("Missing required parameter: token".to_string());
            }
        }
        "auth.getMobileSession" => {
            if !params.contains_key("username") || !params.contains_key("password") {
                return Err("Missing required parameters: username and password".to_string());
            }
        }
        
        _ => {
            return Err(format!("Unknown method: {}", method));
        }
    }
    
    Ok(())
}