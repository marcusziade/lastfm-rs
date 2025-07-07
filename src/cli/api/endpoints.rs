// API endpoint definitions

use serde::{Deserialize, Serialize};

/// API endpoint categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointCategory {
    Artist,
    Album,
    Track,
    Chart,
    Geo,
    Tag,
    User,
    Library,
}

impl EndpointCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Artist => "artist",
            Self::Album => "album",
            Self::Track => "track",
            Self::Chart => "chart",
            Self::Geo => "geo",
            Self::Tag => "tag",
            Self::User => "user",
            Self::Library => "library",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "artist" => Some(Self::Artist),
            "album" => Some(Self::Album),
            "track" => Some(Self::Track),
            "chart" => Some(Self::Chart),
            "geo" => Some(Self::Geo),
            "tag" => Some(Self::Tag),
            "user" => Some(Self::User),
            "library" => Some(Self::Library),
            _ => None,
        }
    }
}

/// API endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub category: String,
    pub method: String,
    pub path: String,
    pub description: String,
    pub requires_auth: bool,
    pub parameters: Vec<Parameter>,
}

/// Parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub param_type: ParameterType,
    pub default: Option<String>,
    pub allowed_values: Option<Vec<String>>,
}

/// Parameter type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Integer,
    Boolean,
    Enum,
}

impl Endpoint {
    /// Create a new endpoint
    pub fn new(
        category: impl Into<String>,
        method: impl Into<String>,
        description: impl Into<String>,
        requires_auth: bool,
    ) -> Self {
        let category = category.into();
        let method = method.into();
        let path = format!("/{}/{}", category, method.replace('.', "/"));
        
        Self {
            category,
            method,
            path,
            description: description.into(),
            requires_auth,
            parameters: Vec::new(),
        }
    }
    
    /// Add a parameter to the endpoint
    pub fn with_param(mut self, param: Parameter) -> Self {
        self.parameters.push(param);
        self
    }
    
    /// Get the full method name (e.g., "artist.getInfo")
    pub fn full_method(&self) -> String {
        format!("{}.{}", self.category, self.method)
    }
}

impl Parameter {
    /// Create a required string parameter
    pub fn required_string(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            required: true,
            param_type: ParameterType::String,
            default: None,
            allowed_values: None,
        }
    }
    
    /// Create an optional string parameter
    pub fn optional_string(
        name: impl Into<String>,
        description: impl Into<String>,
        default: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            required: false,
            param_type: ParameterType::String,
            default,
            allowed_values: None,
        }
    }
    
    /// Create an optional integer parameter
    pub fn optional_integer(
        name: impl Into<String>,
        description: impl Into<String>,
        default: Option<i32>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            required: false,
            param_type: ParameterType::Integer,
            default: default.map(|v| v.to_string()),
            allowed_values: None,
        }
    }
    
    /// Create an optional boolean parameter
    pub fn optional_boolean(
        name: impl Into<String>,
        description: impl Into<String>,
        default: bool,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            required: false,
            param_type: ParameterType::Boolean,
            default: Some(if default { "1" } else { "0" }.to_string()),
            allowed_values: None,
        }
    }
    
    /// Create an enum parameter
    pub fn enum_param(
        name: impl Into<String>,
        description: impl Into<String>,
        allowed_values: Vec<String>,
        default: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            required: default.is_none(),
            param_type: ParameterType::Enum,
            default,
            allowed_values: Some(allowed_values),
        }
    }
}

/// Get all available endpoints
pub fn get_endpoints() -> Vec<Endpoint> {
    vec![
        // Artist endpoints
        Endpoint::new("artist", "getCorrection", "Get corrections for misspelled artist names", false)
            .with_param(Parameter::required_string("artist", "Artist name")),
        
        Endpoint::new("artist", "getInfo", "Get artist metadata including biography", false)
            .with_param(Parameter::optional_string("artist", "Artist name", None))
            .with_param(Parameter::optional_string("mbid", "MusicBrainz ID", None))
            .with_param(Parameter::optional_boolean("autocorrect", "Autocorrect misspellings", false))
            .with_param(Parameter::optional_string("lang", "Language for biography", None))
            .with_param(Parameter::optional_string("username", "Username for context", None)),
        
        Endpoint::new("artist", "getSimilar", "Get similar artists", false)
            .with_param(Parameter::optional_string("artist", "Artist name", None))
            .with_param(Parameter::optional_string("mbid", "MusicBrainz ID", None))
            .with_param(Parameter::optional_integer("limit", "Number of results", Some(50)))
            .with_param(Parameter::optional_boolean("autocorrect", "Autocorrect misspellings", false)),
        
        Endpoint::new("artist", "getTopAlbums", "Get top albums for an artist", false)
            .with_param(Parameter::optional_string("artist", "Artist name", None))
            .with_param(Parameter::optional_string("mbid", "MusicBrainz ID", None))
            .with_param(Parameter::optional_boolean("autocorrect", "Autocorrect misspellings", false))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1)))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50))),
        
        Endpoint::new("artist", "getTopTags", "Get top tags for an artist", false)
            .with_param(Parameter::optional_string("artist", "Artist name", None))
            .with_param(Parameter::optional_string("mbid", "MusicBrainz ID", None))
            .with_param(Parameter::optional_boolean("autocorrect", "Autocorrect misspellings", false)),
        
        Endpoint::new("artist", "getTopTracks", "Get top tracks by an artist", false)
            .with_param(Parameter::optional_string("artist", "Artist name", None))
            .with_param(Parameter::optional_string("mbid", "MusicBrainz ID", None))
            .with_param(Parameter::optional_boolean("autocorrect", "Autocorrect misspellings", false))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1)))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50))),
        
        Endpoint::new("artist", "search", "Search for artists", false)
            .with_param(Parameter::required_string("artist", "Search query"))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(30)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        // Album endpoints
        Endpoint::new("album", "getInfo", "Get album metadata and tracklist", false)
            .with_param(Parameter::optional_string("artist", "Artist name", None))
            .with_param(Parameter::optional_string("album", "Album name", None))
            .with_param(Parameter::optional_string("mbid", "MusicBrainz ID", None))
            .with_param(Parameter::optional_boolean("autocorrect", "Autocorrect misspellings", false))
            .with_param(Parameter::optional_string("username", "Username for context", None))
            .with_param(Parameter::optional_string("lang", "Language for biography", None)),
        
        Endpoint::new("album", "getTopTags", "Get top tags for an album", false)
            .with_param(Parameter::required_string("artist", "Artist name"))
            .with_param(Parameter::required_string("album", "Album name"))
            .with_param(Parameter::optional_string("mbid", "MusicBrainz ID", None))
            .with_param(Parameter::optional_boolean("autocorrect", "Autocorrect misspellings", false)),
        
        Endpoint::new("album", "search", "Search for albums", false)
            .with_param(Parameter::required_string("album", "Search query"))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(30)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        // Track endpoints
        Endpoint::new("track", "getCorrection", "Get corrections for misspelled track names", false)
            .with_param(Parameter::required_string("artist", "Artist name"))
            .with_param(Parameter::required_string("track", "Track name")),
        
        Endpoint::new("track", "getInfo", "Get track metadata", false)
            .with_param(Parameter::optional_string("artist", "Artist name", None))
            .with_param(Parameter::optional_string("track", "Track name", None))
            .with_param(Parameter::optional_string("mbid", "MusicBrainz ID", None))
            .with_param(Parameter::optional_string("username", "Username for context", None))
            .with_param(Parameter::optional_boolean("autocorrect", "Autocorrect misspellings", false)),
        
        Endpoint::new("track", "getSimilar", "Get similar tracks", false)
            .with_param(Parameter::optional_string("artist", "Artist name", None))
            .with_param(Parameter::optional_string("track", "Track name", None))
            .with_param(Parameter::optional_string("mbid", "MusicBrainz ID", None))
            .with_param(Parameter::optional_boolean("autocorrect", "Autocorrect misspellings", false))
            .with_param(Parameter::optional_integer("limit", "Number of results", Some(50))),
        
        Endpoint::new("track", "getTopTags", "Get top tags for a track", false)
            .with_param(Parameter::required_string("artist", "Artist name"))
            .with_param(Parameter::required_string("track", "Track name"))
            .with_param(Parameter::optional_string("mbid", "MusicBrainz ID", None))
            .with_param(Parameter::optional_boolean("autocorrect", "Autocorrect misspellings", false)),
        
        Endpoint::new("track", "search", "Search for tracks", false)
            .with_param(Parameter::required_string("track", "Search query"))
            .with_param(Parameter::optional_string("artist", "Filter by artist", None))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(30)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        // Chart endpoints
        Endpoint::new("chart", "getTopArtists", "Get top artists chart", false)
            .with_param(Parameter::optional_integer("page", "Page number", Some(1)))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50))),
        
        Endpoint::new("chart", "getTopTags", "Get top tags chart", false)
            .with_param(Parameter::optional_integer("page", "Page number", Some(1)))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50))),
        
        Endpoint::new("chart", "getTopTracks", "Get top tracks chart", false)
            .with_param(Parameter::optional_integer("page", "Page number", Some(1)))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50))),
        
        // Geo endpoints
        Endpoint::new("geo", "getTopArtists", "Get top artists by country", false)
            .with_param(Parameter::required_string("country", "Country name (ISO 3166-1)"))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1)))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50))),
        
        Endpoint::new("geo", "getTopTracks", "Get top tracks by country", false)
            .with_param(Parameter::required_string("country", "Country name (ISO 3166-1)"))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1)))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50))),
        
        // Tag endpoints
        Endpoint::new("tag", "getInfo", "Get tag information", false)
            .with_param(Parameter::required_string("tag", "Tag name"))
            .with_param(Parameter::optional_string("lang", "Language for summary", None)),
        
        Endpoint::new("tag", "getSimilar", "Get similar tags", false)
            .with_param(Parameter::required_string("tag", "Tag name")),
        
        Endpoint::new("tag", "getTopAlbums", "Get top albums for a tag", false)
            .with_param(Parameter::required_string("tag", "Tag name"))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        Endpoint::new("tag", "getTopArtists", "Get top artists for a tag", false)
            .with_param(Parameter::required_string("tag", "Tag name"))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        Endpoint::new("tag", "getTopTags", "Get global top tags", false),
        
        Endpoint::new("tag", "getTopTracks", "Get top tracks for a tag", false)
            .with_param(Parameter::required_string("tag", "Tag name"))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        Endpoint::new("tag", "getWeeklyChartList", "Get available charts for a tag", false)
            .with_param(Parameter::required_string("tag", "Tag name")),
        
        // User endpoints (public methods only)
        Endpoint::new("user", "getFriends", "Get a user's friends", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::optional_boolean("recenttracks", "Include recent tracks", false))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        Endpoint::new("user", "getInfo", "Get user profile information", false)
            .with_param(Parameter::optional_string("user", "Username", None)),
        
        Endpoint::new("user", "getLovedTracks", "Get a user's loved tracks", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        Endpoint::new("user", "getPersonalTags", "Get a user's personal tags", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::required_string("tag", "Tag name"))
            .with_param(Parameter::enum_param(
                "taggingtype",
                "Type of items tagged",
                vec!["artist".to_string(), "album".to_string(), "track".to_string()],
                None,
            ))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        Endpoint::new("user", "getRecentTracks", "Get a user's recent tracks", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::optional_integer("limit", "Results per page (max 200)", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1)))
            .with_param(Parameter::optional_integer("from", "Beginning timestamp", None))
            .with_param(Parameter::optional_integer("to", "End timestamp", None))
            .with_param(Parameter::optional_boolean("extended", "Include extended data", false)),
        
        Endpoint::new("user", "getTopAlbums", "Get a user's top albums", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::enum_param(
                "period",
                "Time period",
                vec![
                    "overall".to_string(),
                    "7day".to_string(),
                    "1month".to_string(),
                    "3month".to_string(),
                    "6month".to_string(),
                    "12month".to_string(),
                ],
                Some("overall".to_string()),
            ))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        Endpoint::new("user", "getTopArtists", "Get a user's top artists", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::enum_param(
                "period",
                "Time period",
                vec![
                    "overall".to_string(),
                    "7day".to_string(),
                    "1month".to_string(),
                    "3month".to_string(),
                    "6month".to_string(),
                    "12month".to_string(),
                ],
                Some("overall".to_string()),
            ))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        Endpoint::new("user", "getTopTags", "Get a user's top tags", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::optional_integer("limit", "Number of results", Some(50))),
        
        Endpoint::new("user", "getTopTracks", "Get a user's top tracks", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::enum_param(
                "period",
                "Time period",
                vec![
                    "overall".to_string(),
                    "7day".to_string(),
                    "1month".to_string(),
                    "3month".to_string(),
                    "6month".to_string(),
                    "12month".to_string(),
                ],
                Some("overall".to_string()),
            ))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
        
        Endpoint::new("user", "getWeeklyAlbumChart", "Get a user's weekly album chart", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::optional_integer("from", "Start timestamp", None))
            .with_param(Parameter::optional_integer("to", "End timestamp", None)),
        
        Endpoint::new("user", "getWeeklyArtistChart", "Get a user's weekly artist chart", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::optional_integer("from", "Start timestamp", None))
            .with_param(Parameter::optional_integer("to", "End timestamp", None)),
        
        Endpoint::new("user", "getWeeklyChartList", "Get available charts for a user", false)
            .with_param(Parameter::required_string("user", "Username")),
        
        Endpoint::new("user", "getWeeklyTrackChart", "Get a user's weekly track chart", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::optional_integer("from", "Start timestamp", None))
            .with_param(Parameter::optional_integer("to", "End timestamp", None)),
        
        // Library endpoints
        Endpoint::new("library", "getArtists", "Get all artists in a user's library", false)
            .with_param(Parameter::required_string("user", "Username"))
            .with_param(Parameter::optional_integer("limit", "Results per page", Some(50)))
            .with_param(Parameter::optional_integer("page", "Page number", Some(1))),
    ]
}