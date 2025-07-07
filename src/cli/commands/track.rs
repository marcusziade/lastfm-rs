// Track command implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::cli::{
    error::{CliError, Result},
    traits::{ApiClient, Command, CommandArgs, CommandOutput},
};

use super::{BaseCommand, get_required_arg, get_optional_arg};

/// Get track information
pub struct TrackInfoCommand {
    base: BaseCommand,
}

impl TrackInfoCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "track.info",
                "Get track metadata",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for TrackInfoCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        // Get artist and track or mbid
        if let Some(mbid) = args.named.get("mbid") {
            params.insert("mbid".to_string(), mbid.clone());
        } else {
            let artist = get_required_arg(args, "artist")?;
            let track = args.named.get("track")
                .cloned()
                .or_else(|| args.positional.get(1).cloned())
                .ok_or_else(|| CliError::missing_argument("track"))?;
            
            params.insert("artist".to_string(), artist);
            params.insert("track".to_string(), track);
        }
        
        self.base.execute_api_call("/track/getInfo", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if !args.named.contains_key("mbid") {
            if args.positional.len() < 2 && 
               (!args.named.contains_key("artist") || !args.named.contains_key("track")) {
                return Err(CliError::validation("Either mbid or both artist and track are required"));
            }
        }
        Ok(())
    }
}

/// Get similar tracks
pub struct TrackSimilarCommand {
    base: BaseCommand,
}

impl TrackSimilarCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "track.similar",
                "Get similar tracks",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for TrackSimilarCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        // Similar to track.info
        if let Some(mbid) = args.named.get("mbid") {
            params.insert("mbid".to_string(), mbid.clone());
        } else {
            let artist = get_required_arg(args, "artist")?;
            let track = args.named.get("track")
                .cloned()
                .or_else(|| args.positional.get(1).cloned())
                .ok_or_else(|| CliError::missing_argument("track"))?;
            
            params.insert("artist".to_string(), artist);
            params.insert("track".to_string(), track);
        }
        
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        
        self.base.execute_api_call("/track/getSimilar", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if !args.named.contains_key("mbid") {
            if args.positional.len() < 2 && 
               (!args.named.contains_key("artist") || !args.named.contains_key("track")) {
                return Err(CliError::validation("Either mbid or both artist and track are required"));
            }
        }
        Ok(())
    }
}

/// Search for tracks
pub struct TrackSearchCommand {
    base: BaseCommand,
}

impl TrackSearchCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "track.search",
                "Search for tracks",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for TrackSearchCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        let query = get_required_arg(args, "query")?;
        params.insert("track".to_string(), query);
        
        if let Some(artist) = args.named.get("artist") {
            params.insert("artist".to_string(), artist.clone());
        }
        
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("30")),
        );
        
        self.base.execute_api_call("/track/search", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if args.positional.is_empty() && !args.named.contains_key("query") {
            return Err(CliError::missing_argument("query"));
        }
        Ok(())
    }
}