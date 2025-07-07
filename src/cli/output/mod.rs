// Output formatting module

use colored::*;
use prettytable::{row, Table};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cli::traits::{CommandOutput, OutputFormatter as OutputFormatterTrait};

#[cfg(test)]
mod tests;

/// Output format types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Table,
    Pretty,
    Compact,
}

/// JSON output formatter
pub struct JsonFormatter {
    pretty: bool,
}

impl JsonFormatter {
    pub fn new(pretty: bool) -> Self {
        Self { pretty }
    }
}

impl OutputFormatterTrait for JsonFormatter {
    fn format(&self, output: &CommandOutput) -> String {
        if self.pretty {
            serde_json::to_string_pretty(&output.data).unwrap_or_else(|e| e.to_string())
        } else {
            serde_json::to_string(&output.data).unwrap_or_else(|e| e.to_string())
        }
    }
    
    fn name(&self) -> &str {
        if self.pretty {
            "json-pretty"
        } else {
            "json-compact"
        }
    }
}

/// Table output formatter
pub struct TableFormatter;

impl OutputFormatterTrait for TableFormatter {
    fn format(&self, output: &CommandOutput) -> String {
        let mut result = String::new();
        
        // Add metadata if available
        if output.metadata.api_calls_made > 0 {
            result.push_str(&format!(
                "Response time: {}ms | Cache: {} | API calls: {}\n\n",
                output.metadata.response_time_ms,
                if output.metadata.cache_hit { "HIT" } else { "MISS" },
                output.metadata.api_calls_made
            ));
        }
        
        // Format the data as a table
        if let Some(formatted) = format_value_as_table(&output.data) {
            result.push_str(&formatted);
        } else {
            result.push_str(&serde_json::to_string_pretty(&output.data).unwrap_or_default());
        }
        
        result
    }
    
    fn name(&self) -> &str {
        "table"
    }
}

/// Pretty output formatter
pub struct PrettyFormatter {
    color: bool,
}

impl PrettyFormatter {
    pub fn new(color: bool) -> Self {
        Self { color }
    }
}

impl OutputFormatterTrait for PrettyFormatter {
    fn format(&self, output: &CommandOutput) -> String {
        let mut result = String::new();
        
        // Add metadata header if available
        if output.metadata.api_calls_made > 0 {
            let metadata = format!(
                "⏱  {}ms | 📦 {} | 🌐 {} API call{}",
                output.metadata.response_time_ms,
                if output.metadata.cache_hit { "Cached" } else { "Fresh" },
                output.metadata.api_calls_made,
                if output.metadata.api_calls_made == 1 { "" } else { "s" }
            );
            
            if self.color {
                result.push_str(&metadata.dimmed().to_string());
            } else {
                result.push_str(&metadata);
            }
            result.push_str("\n\n");
        }
        
        // Format the response data
        if let Some(formatted) = format_value_pretty(&output.data, self.color) {
            result.push_str(&formatted);
        } else {
            result.push_str(&serde_json::to_string_pretty(&output.data).unwrap_or_default());
        }
        
        result
    }
    
    fn name(&self) -> &str {
        "pretty"
    }
}

/// Format a JSON value as a table
fn format_value_as_table(value: &Value) -> Option<String> {
    match value {
        Value::Object(map) => {
            // Special handling for search results
            if let Some(results) = map.get("results") {
                if let Some(artist_matches) = results.get("artistmatches") {
                    return format_search_artists_table(artist_matches);
                } else if let Some(album_matches) = results.get("albummatches") {
                    return format_search_albums_table(album_matches);
                } else if let Some(track_matches) = results.get("trackmatches") {
                    return format_search_tracks_table(track_matches);
                }
            }
            
            // Special handling for different response types
            if map.contains_key("artist") {
                format_artist_table(map.get("artist")?)
            } else if map.contains_key("album") {
                format_album_table(map.get("album")?)
            } else if map.contains_key("track") {
                format_track_table(map.get("track")?)
            } else if map.contains_key("artists") {
                format_artists_list_table(map.get("artists")?)
            } else if map.contains_key("albums") {
                format_albums_list_table(map.get("albums")?)
            } else if map.contains_key("tracks") {
                format_tracks_list_table(map.get("tracks")?)
            } else if let Some(topartists) = map.get("topartists") {
                format_top_artists_table(topartists)
            } else if let Some(topalbums) = map.get("topalbums") {
                format_top_albums_table(topalbums)
            } else if let Some(toptracks) = map.get("toptracks") {
                format_top_tracks_table(toptracks)
            } else if let Some(tags) = map.get("tags") {
                format_tags_table(tags)
            } else if let Some(similartracks) = map.get("similartracks") {
                format_similar_tracks_table(similartracks)
            } else if let Some(similarartists) = map.get("similarartists") {
                format_similar_artists_table(similarartists)
            } else if let Some(recenttracks) = map.get("recenttracks") {
                format_recent_tracks_table(recenttracks)
            } else {
                format_generic_object_table(map)
            }
        }
        Value::Array(arr) => format_array_table(arr),
        _ => None,
    }
}

/// Format an artist response as a table
fn format_artist_table(artist: &Value) -> Option<String> {
    let mut table = Table::new();
    table.add_row(row!["Field", "Value"]);
    
    if let Some(name) = artist.get("name") {
        table.add_row(row!["Name", name.as_str().unwrap_or("")]);
    }
    
    if let Some(listeners) = artist.get("stats").and_then(|s| s.get("listeners")) {
        table.add_row(row!["Listeners", listeners]);
    }
    
    if let Some(playcount) = artist.get("stats").and_then(|s| s.get("playcount")) {
        table.add_row(row!["Play Count", playcount]);
    }
    
    if let Some(url) = artist.get("url") {
        table.add_row(row!["URL", url.as_str().unwrap_or("")]);
    }
    
    Some(table.to_string())
}

/// Format an album response as a table
fn format_album_table(album: &Value) -> Option<String> {
    let mut table = Table::new();
    table.add_row(row!["Field", "Value"]);
    
    if let Some(name) = album.get("name") {
        table.add_row(row!["Album", name.as_str().unwrap_or("")]);
    }
    
    if let Some(artist) = album.get("artist") {
        table.add_row(row!["Artist", artist.as_str().unwrap_or("")]);
    }
    
    if let Some(listeners) = album.get("listeners") {
        table.add_row(row!["Listeners", listeners]);
    }
    
    if let Some(playcount) = album.get("playcount") {
        table.add_row(row!["Play Count", playcount]);
    }
    
    Some(table.to_string())
}

/// Format a track response as a table
fn format_track_table(track: &Value) -> Option<String> {
    let mut table = Table::new();
    table.add_row(row!["Field", "Value"]);
    
    if let Some(name) = track.get("name") {
        table.add_row(row!["Track", name.as_str().unwrap_or("")]);
    }
    
    if let Some(artist) = track.get("artist").and_then(|a| a.get("name")) {
        table.add_row(row!["Artist", artist.as_str().unwrap_or("")]);
    }
    
    if let Some(listeners) = track.get("listeners") {
        table.add_row(row!["Listeners", listeners]);
    }
    
    if let Some(playcount) = track.get("playcount") {
        table.add_row(row!["Play Count", playcount]);
    }
    
    Some(table.to_string())
}

/// Format a list of artists as a table
fn format_artists_list_table(artists: &Value) -> Option<String> {
    let artists = artists.get("artist")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Artist", "Listeners", "Play Count"]);
    
    for (i, artist) in artists.iter().enumerate() {
        let name = artist.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let listeners = artist.get("listeners").and_then(|l| l.as_str()).unwrap_or("0");
        let playcount = artist.get("playcount").and_then(|p| p.as_str()).unwrap_or("0");
        
        table.add_row(row![i + 1, name, listeners, playcount]);
    }
    
    Some(table.to_string())
}

/// Format a list of albums as a table
fn format_albums_list_table(albums: &Value) -> Option<String> {
    let albums = albums.get("album")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Album", "Artist", "Play Count"]);
    
    for (i, album) in albums.iter().enumerate() {
        let name = album.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let artist = album.get("artist").and_then(|a| a.get("name")).and_then(|n| n.as_str()).unwrap_or("");
        let playcount = album.get("playcount").and_then(|p| p.as_str()).unwrap_or("0");
        
        table.add_row(row![i + 1, name, artist, playcount]);
    }
    
    Some(table.to_string())
}

/// Format a list of tracks as a table
fn format_tracks_list_table(tracks: &Value) -> Option<String> {
    let tracks = tracks.get("track")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Track", "Artist", "Listeners"]);
    
    for (i, track) in tracks.iter().enumerate() {
        let name = track.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let artist = match track.get("artist") {
            Some(Value::String(s)) => s.as_str(),
            Some(Value::Object(obj)) => obj.get("name").and_then(|n| n.as_str()).unwrap_or(""),
            _ => ""
        };
        let listeners = track.get("listeners").and_then(|l| l.as_str()).unwrap_or("0");
        
        table.add_row(row![i + 1, name, artist, listeners]);
    }
    
    Some(table.to_string())
}

/// Format a generic object as a table
fn format_generic_object_table(map: &serde_json::Map<String, Value>) -> Option<String> {
    let mut table = Table::new();
    table.add_row(row!["Key", "Value"]);
    
    for (key, value) in map {
        let value_str = match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            _ => serde_json::to_string(value).unwrap_or_default(),
        };
        
        table.add_row(row![key, value_str]);
    }
    
    Some(table.to_string())
}

/// Format an array as a table
fn format_array_table(arr: &[Value]) -> Option<String> {
    if arr.is_empty() {
        return Some("Empty array".to_string());
    }
    
    let mut table = Table::new();
    table.add_row(row!["Index", "Value"]);
    
    for (i, value) in arr.iter().enumerate() {
        let value_str = match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            _ => serde_json::to_string(value).unwrap_or_default(),
        };
        
        table.add_row(row![i, value_str]);
    }
    
    Some(table.to_string())
}

/// Format search artists results as a table
fn format_search_artists_table(matches: &Value) -> Option<String> {
    let artists = matches.get("artist")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Artist", "Listeners", "URL"]);
    
    for (i, artist) in artists.iter().enumerate() {
        let name = artist.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let listeners = artist.get("listeners").and_then(|l| l.as_str()).unwrap_or("0");
        let url = artist.get("url").and_then(|u| u.as_str()).unwrap_or("");
        
        table.add_row(row![i + 1, name, listeners, url]);
    }
    
    Some(table.to_string())
}

/// Format search albums results as a table
fn format_search_albums_table(matches: &Value) -> Option<String> {
    let albums = matches.get("album")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Album", "Artist", "URL"]);
    
    for (i, album) in albums.iter().enumerate() {
        let name = album.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let artist = album.get("artist").and_then(|a| a.as_str()).unwrap_or("");
        let url = album.get("url").and_then(|u| u.as_str()).unwrap_or("");
        
        table.add_row(row![i + 1, name, artist, url]);
    }
    
    Some(table.to_string())
}

/// Format search tracks results as a table
fn format_search_tracks_table(matches: &Value) -> Option<String> {
    let tracks = matches.get("track")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Track", "Artist", "Listeners"]);
    
    for (i, track) in tracks.iter().enumerate() {
        let name = track.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let artist = track.get("artist").and_then(|a| a.as_str()).unwrap_or("");
        let listeners = track.get("listeners").and_then(|l| l.as_str()).unwrap_or("0");
        
        table.add_row(row![i + 1, name, artist, listeners]);
    }
    
    Some(table.to_string())
}

/// Format top artists response as a table
fn format_top_artists_table(data: &Value) -> Option<String> {
    let artists = data.get("artist")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Artist", "Play Count", "Listeners"]);
    
    for (i, artist) in artists.iter().enumerate() {
        let name = artist.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let playcount = artist.get("playcount").and_then(|p| p.as_str()).unwrap_or("0");
        let listeners = artist.get("listeners").and_then(|l| l.as_str()).unwrap_or("0");
        
        table.add_row(row![i + 1, name, playcount, listeners]);
    }
    
    Some(table.to_string())
}

/// Format top albums response as a table
fn format_top_albums_table(data: &Value) -> Option<String> {
    let albums = data.get("album")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Album", "Artist", "Play Count"]);
    
    for (i, album) in albums.iter().enumerate() {
        let name = album.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let artist = album.get("artist").and_then(|a| a.get("name")).and_then(|n| n.as_str()).unwrap_or("");
        let playcount = album.get("playcount").and_then(|p| p.as_str()).unwrap_or("0");
        
        table.add_row(row![i + 1, name, artist, playcount]);
    }
    
    Some(table.to_string())
}

/// Format top tracks response as a table
fn format_top_tracks_table(data: &Value) -> Option<String> {
    let tracks = data.get("track")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Track", "Artist", "Play Count"]);
    
    for (i, track) in tracks.iter().enumerate() {
        let name = track.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let artist = match track.get("artist") {
            Some(Value::String(s)) => s.as_str(),
            Some(Value::Object(obj)) => obj.get("name").and_then(|n| n.as_str()).unwrap_or(""),
            _ => ""
        };
        let playcount = track.get("playcount").and_then(|p| p.as_str()).unwrap_or("0");
        
        table.add_row(row![i + 1, name, artist, playcount]);
    }
    
    Some(table.to_string())
}

/// Format tags response as a table
fn format_tags_table(data: &Value) -> Option<String> {
    let tags = data.get("tag")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Tag", "Taggings", "Reach"]);
    
    for (i, tag) in tags.iter().enumerate() {
        let name = tag.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let taggings = tag.get("taggings").and_then(|t| t.as_str()).unwrap_or("0");
        let reach = tag.get("reach").and_then(|r| r.as_str()).unwrap_or("0");
        
        table.add_row(row![i + 1, name, taggings, reach]);
    }
    
    Some(table.to_string())
}

/// Format similar tracks response as a table
fn format_similar_tracks_table(data: &Value) -> Option<String> {
    let tracks = data.get("track")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Track", "Artist", "Match %", "Play Count"]);
    
    for (i, track) in tracks.iter().enumerate() {
        let name = track.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let artist = track.get("artist")
            .and_then(|a| a.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("");
        let match_score = track.get("match")
            .and_then(|m| m.as_f64())
            .map(|m| format!("{:.1}", m * 100.0))
            .unwrap_or_else(|| "0.0".to_string());
        let playcount = track.get("playcount")
            .and_then(|p| p.as_u64())
            .map(|p| p.to_string())
            .unwrap_or_else(|| "0".to_string());
        
        table.add_row(row![i + 1, name, artist, match_score, playcount]);
    }
    
    Some(table.to_string())
}

/// Format similar artists response as a table
fn format_similar_artists_table(data: &Value) -> Option<String> {
    let artists = data.get("artist")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Artist", "Match %"]);
    
    for (i, artist) in artists.iter().enumerate() {
        let name = artist.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let match_score = artist.get("match")
            .and_then(|m| m.as_str())
            .and_then(|m| m.parse::<f64>().ok())
            .map(|m| format!("{:.1}", m * 100.0))
            .unwrap_or_else(|| "0.0".to_string());
        
        table.add_row(row![i + 1, name, match_score]);
    }
    
    Some(table.to_string())
}

/// Format recent tracks response as a table
fn format_recent_tracks_table(data: &Value) -> Option<String> {
    let tracks = data.get("track")?.as_array()?;
    let mut table = Table::new();
    table.add_row(row!["#", "Track", "Artist", "Album", "Date"]);
    
    for (i, track) in tracks.iter().enumerate() {
        let name = track.get("name").and_then(|n| n.as_str()).unwrap_or("");
        
        // Handle artist field which can be string or object
        let artist = match track.get("artist") {
            Some(Value::String(s)) => s.as_str(),
            Some(Value::Object(obj)) => obj.get("name").and_then(|n| n.as_str()).unwrap_or(""),
            _ => ""
        };
        
        // Get album info
        let album = match track.get("album") {
            Some(Value::Object(obj)) => obj.get("#text").and_then(|t| t.as_str()).unwrap_or(""),
            _ => ""
        };
        
        // Get date/time info
        let date = match track.get("date") {
            Some(Value::Object(obj)) => {
                let uts = obj.get("uts").and_then(|u| u.as_str()).unwrap_or("");
                let text = obj.get("#text").and_then(|t| t.as_str()).unwrap_or("");
                if !text.is_empty() {
                    text
                } else {
                    uts
                }
            },
            _ => {
                // Check if currently playing
                if track.get("@attr").and_then(|a| a.get("nowplaying")).and_then(|n| n.as_str()) == Some("true") {
                    "Now Playing"
                } else {
                    ""
                }
            }
        };
        
        table.add_row(row![i + 1, name, artist, album, date]);
    }
    
    Some(table.to_string())
}

/// Format a JSON value in pretty format
fn format_value_pretty(value: &Value, color: bool) -> Option<String> {
    match value {
        Value::Object(map) => {
            // Check for error response
            if let Some(error) = map.get("error") {
                return Some(format_error_pretty(error, map.get("message"), color));
            }
            
            // Special handling for different response types
            if map.contains_key("artist") {
                format_artist_pretty(map.get("artist")?, color)
            } else if map.contains_key("album") {
                format_album_pretty(map.get("album")?, color)
            } else if map.contains_key("track") {
                format_track_pretty(map.get("track")?, color)
            } else if map.contains_key("topartists") {
                format_top_artists_pretty(map.get("topartists")?, color)
            } else if map.contains_key("topalbums") {
                format_top_albums_pretty(map.get("topalbums")?, color)
            } else if map.contains_key("toptracks") {
                format_top_tracks_pretty(map.get("toptracks")?, color)
            } else if map.contains_key("results") {
                format_search_results_pretty(map.get("results")?, color)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Format an error response
fn format_error_pretty(error: &Value, message: Option<&Value>, color: bool) -> String {
    let error_code = error.as_u64().unwrap_or(0);
    let error_msg = message
        .and_then(|m| m.as_str())
        .unwrap_or("Unknown error");
    
    if color {
        format!(
            "{} {} {}\n{}",
            "Error".red().bold(),
            error_code.to_string().red(),
            "-".dimmed(),
            error_msg.yellow()
        )
    } else {
        format!("Error {} - {}", error_code, error_msg)
    }
}

/// Format an artist response in pretty format
fn format_artist_pretty(artist: &Value, color: bool) -> Option<String> {
    let mut result = String::new();
    
    if let Some(name) = artist.get("name").and_then(|n| n.as_str()) {
        if color {
            result.push_str(&format!("{}\n", name.green().bold()));
        } else {
            result.push_str(&format!("{}\n", name));
        }
    }
    
    if let Some(url) = artist.get("url").and_then(|u| u.as_str()) {
        if color {
            result.push_str(&format!("🔗 {}\n", url.blue()));
        } else {
            result.push_str(&format!("URL: {}\n", url));
        }
    }
    
    if let Some(stats) = artist.get("stats") {
        if let Some(listeners) = stats.get("listeners") {
            if color {
                result.push_str(&format!("👥 {} listeners\n", listeners.to_string().cyan()));
            } else {
                result.push_str(&format!("Listeners: {}\n", listeners));
            }
        }
        
        if let Some(playcount) = stats.get("playcount") {
            if color {
                result.push_str(&format!("▶️  {} plays\n", playcount.to_string().cyan()));
            } else {
                result.push_str(&format!("Plays: {}\n", playcount));
            }
        }
    }
    
    if let Some(bio) = artist.get("bio").and_then(|b| b.get("summary")).and_then(|s| s.as_str()) {
        result.push_str("\n");
        if color {
            result.push_str(&"Biography:".yellow().bold().to_string());
        } else {
            result.push_str("Biography:");
        }
        result.push_str("\n");
        
        // Clean up HTML tags
        let clean_bio = bio
            .replace("<![CDATA[", "")
            .replace("]]>", "")
            .replace("<a href=", "")
            .replace("</a>", "")
            .replace(">", " ");
        
        result.push_str(&clean_bio);
        result.push_str("\n");
    }
    
    if let Some(similar) = artist.get("similar").and_then(|s| s.get("artist")).and_then(|a| a.as_array()) {
        if !similar.is_empty() {
            result.push_str("\n");
            if color {
                result.push_str(&"Similar Artists:".yellow().bold().to_string());
            } else {
                result.push_str("Similar Artists:");
            }
            result.push_str("\n");
            
            for (i, similar_artist) in similar.iter().take(5).enumerate() {
                if let Some(name) = similar_artist.get("name").and_then(|n| n.as_str()) {
                    result.push_str(&format!("  {}. {}\n", i + 1, name));
                }
            }
        }
    }
    
    Some(result)
}

/// Format an album response in pretty format
fn format_album_pretty(album: &Value, color: bool) -> Option<String> {
    let mut result = String::new();
    
    if let Some(name) = album.get("name").and_then(|n| n.as_str()) {
        if color {
            result.push_str(&format!("{}\n", name.green().bold()));
        } else {
            result.push_str(&format!("{}\n", name));
        }
    }
    
    if let Some(artist) = album.get("artist").and_then(|a| a.as_str()) {
        if color {
            result.push_str(&format!("by {}\n", artist.blue()));
        } else {
            result.push_str(&format!("by {}\n", artist));
        }
    }
    
    if let Some(listeners) = album.get("listeners") {
        if color {
            result.push_str(&format!("👥 {} listeners\n", listeners.to_string().cyan()));
        } else {
            result.push_str(&format!("Listeners: {}\n", listeners));
        }
    }
    
    if let Some(tracks) = album.get("tracks").and_then(|t| t.get("track")).and_then(|t| t.as_array()) {
        result.push_str("\n");
        if color {
            result.push_str(&"Tracks:".yellow().bold().to_string());
        } else {
            result.push_str("Tracks:");
        }
        result.push_str("\n");
        
        for (i, track) in tracks.iter().enumerate() {
            if let Some(name) = track.get("name").and_then(|n| n.as_str()) {
                let duration = track.get("duration")
                    .and_then(|d| d.as_str())
                    .and_then(|d| d.parse::<u64>().ok())
                    .map(|d| format!(" ({}:{:02})", d / 60, d % 60))
                    .unwrap_or_default();
                
                result.push_str(&format!("  {:2}. {}{}\n", i + 1, name, duration));
            }
        }
    }
    
    Some(result)
}

/// Format a track response in pretty format
fn format_track_pretty(track: &Value, color: bool) -> Option<String> {
    let mut result = String::new();
    
    if let Some(name) = track.get("name").and_then(|n| n.as_str()) {
        if color {
            result.push_str(&format!("{}\n", name.green().bold()));
        } else {
            result.push_str(&format!("{}\n", name));
        }
    }
    
    if let Some(artist) = track.get("artist").and_then(|a| a.get("name")).and_then(|n| n.as_str()) {
        if color {
            result.push_str(&format!("by {}\n", artist.blue()));
        } else {
            result.push_str(&format!("by {}\n", artist));
        }
    }
    
    if let Some(album) = track.get("album").and_then(|a| a.get("title")).and_then(|t| t.as_str()) {
        if color {
            result.push_str(&format!("from {}\n", album.magenta()));
        } else {
            result.push_str(&format!("from {}\n", album));
        }
    }
    
    if let Some(duration) = track.get("duration").and_then(|d| d.as_str()).and_then(|d| d.parse::<u64>().ok()) {
        let minutes = duration / 60000;
        let seconds = (duration % 60000) / 1000;
        result.push_str(&format!("⏱  {}:{:02}\n", minutes, seconds));
    }
    
    if let Some(listeners) = track.get("listeners") {
        if color {
            result.push_str(&format!("👥 {} listeners\n", listeners.to_string().cyan()));
        } else {
            result.push_str(&format!("Listeners: {}\n", listeners));
        }
    }
    
    if let Some(playcount) = track.get("playcount") {
        if color {
            result.push_str(&format!("▶️  {} plays\n", playcount.to_string().cyan()));
        } else {
            result.push_str(&format!("Plays: {}\n", playcount));
        }
    }
    
    Some(result)
}

/// Format top artists response
fn format_top_artists_pretty(data: &Value, color: bool) -> Option<String> {
    let mut result = String::new();
    
    if color {
        result.push_str(&"Top Artists\n".green().bold().to_string());
    } else {
        result.push_str("Top Artists\n");
    }
    result.push_str(&"─".repeat(50));
    result.push_str("\n");
    
    if let Some(artists) = data.get("artist").and_then(|a| a.as_array()) {
        for (i, artist) in artists.iter().enumerate() {
            let name = artist.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let playcount = artist.get("playcount").and_then(|p| p.as_str()).unwrap_or("0");
            
            if color {
                result.push_str(&format!(
                    "{:2}. {} - {} plays\n",
                    i + 1,
                    name.blue(),
                    playcount.cyan()
                ));
            } else {
                result.push_str(&format!("{:2}. {} - {} plays\n", i + 1, name, playcount));
            }
        }
    }
    
    Some(result)
}

/// Format top albums response
fn format_top_albums_pretty(data: &Value, color: bool) -> Option<String> {
    let mut result = String::new();
    
    if color {
        result.push_str(&"Top Albums\n".green().bold().to_string());
    } else {
        result.push_str("Top Albums\n");
    }
    result.push_str(&"─".repeat(50));
    result.push_str("\n");
    
    if let Some(albums) = data.get("album").and_then(|a| a.as_array()) {
        for (i, album) in albums.iter().enumerate() {
            let name = album.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let artist = album.get("artist").and_then(|a| a.get("name")).and_then(|n| n.as_str()).unwrap_or("");
            let playcount = album.get("playcount").and_then(|p| p.as_str()).unwrap_or("0");
            
            if color {
                result.push_str(&format!(
                    "{:2}. {} by {} - {} plays\n",
                    i + 1,
                    name.blue(),
                    artist.magenta(),
                    playcount.cyan()
                ));
            } else {
                result.push_str(&format!("{:2}. {} by {} - {} plays\n", i + 1, name, artist, playcount));
            }
        }
    }
    
    Some(result)
}

/// Format top tracks response
fn format_top_tracks_pretty(data: &Value, color: bool) -> Option<String> {
    let mut result = String::new();
    
    if color {
        result.push_str(&"Top Tracks\n".green().bold().to_string());
    } else {
        result.push_str("Top Tracks\n");
    }
    result.push_str(&"─".repeat(50));
    result.push_str("\n");
    
    if let Some(tracks) = data.get("track").and_then(|t| t.as_array()) {
        for (i, track) in tracks.iter().enumerate() {
            let name = track.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let artist = track.get("artist").and_then(|a| a.get("name")).and_then(|n| n.as_str()).unwrap_or("");
            let playcount = track.get("playcount").and_then(|p| p.as_str()).unwrap_or("0");
            
            if color {
                result.push_str(&format!(
                    "{:2}. {} by {} - {} plays\n",
                    i + 1,
                    name.blue(),
                    artist.magenta(),
                    playcount.cyan()
                ));
            } else {
                result.push_str(&format!("{:2}. {} by {} - {} plays\n", i + 1, name, artist, playcount));
            }
        }
    }
    
    Some(result)
}

/// Format search results
fn format_search_results_pretty(results: &Value, color: bool) -> Option<String> {
    let mut result = String::new();
    
    // Determine the type of search results
    if let Some(artist_matches) = results.get("artistmatches") {
        if color {
            result.push_str(&"Artist Search Results\n".green().bold().to_string());
        } else {
            result.push_str("Artist Search Results\n");
        }
        result.push_str(&"─".repeat(50));
        result.push_str("\n");
        
        if let Some(artists) = artist_matches.get("artist").and_then(|a| a.as_array()) {
            for artist in artists.iter() {
                let name = artist.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let listeners = artist.get("listeners").and_then(|l| l.as_str()).unwrap_or("0");
                
                if color {
                    result.push_str(&format!("• {} - {} listeners\n", name.blue(), listeners.cyan()));
                } else {
                    result.push_str(&format!("• {} - {} listeners\n", name, listeners));
                }
            }
        }
    } else if let Some(album_matches) = results.get("albummatches") {
        if color {
            result.push_str(&"Album Search Results\n".green().bold().to_string());
        } else {
            result.push_str("Album Search Results\n");
        }
        result.push_str(&"─".repeat(50));
        result.push_str("\n");
        
        if let Some(albums) = album_matches.get("album").and_then(|a| a.as_array()) {
            for album in albums.iter() {
                let name = album.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let artist = album.get("artist").and_then(|a| a.as_str()).unwrap_or("");
                
                if color {
                    result.push_str(&format!("• {} by {}\n", name.blue(), artist.magenta()));
                } else {
                    result.push_str(&format!("• {} by {}\n", name, artist));
                }
            }
        }
    } else if let Some(track_matches) = results.get("trackmatches") {
        if color {
            result.push_str(&"Track Search Results\n".green().bold().to_string());
        } else {
            result.push_str("Track Search Results\n");
        }
        result.push_str(&"─".repeat(50));
        result.push_str("\n");
        
        if let Some(tracks) = track_matches.get("track").and_then(|t| t.as_array()) {
            for track in tracks.iter() {
                let name = track.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let artist = track.get("artist").and_then(|a| a.as_str()).unwrap_or("");
                let listeners = track.get("listeners").and_then(|l| l.as_str()).unwrap_or("0");
                
                if color {
                    result.push_str(&format!(
                        "• {} by {} - {} listeners\n",
                        name.blue(),
                        artist.magenta(),
                        listeners.cyan()
                    ));
                } else {
                    result.push_str(&format!("• {} by {} - {} listeners\n", name, artist, listeners));
                }
            }
        }
    }
    
    Some(result)
}

/// Create an output formatter based on the format type
pub fn create_formatter(format: OutputFormat, color: bool) -> Box<dyn OutputFormatterTrait> {
    match format {
        OutputFormat::Json => Box::new(JsonFormatter::new(true)),
        OutputFormat::Compact => Box::new(JsonFormatter::new(false)),
        OutputFormat::Table => Box::new(TableFormatter),
        OutputFormat::Pretty => Box::new(PrettyFormatter::new(color)),
    }
}