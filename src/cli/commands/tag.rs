// Tag command implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::cli::{
    error::{CliError, Result},
    traits::{ApiClient, Command, CommandArgs, CommandOutput},
};

use super::{BaseCommand, get_required_arg, get_optional_arg};

/// Get tag information
pub struct TagInfoCommand {
    base: BaseCommand,
}

impl TagInfoCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "tag.info",
                "Get tag information",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for TagInfoCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        let tag = get_required_arg(args, "tag")?;
        params.insert("tag".to_string(), tag);
        
        if let Some(lang) = args.named.get("lang") {
            params.insert("lang".to_string(), lang.clone());
        }
        
        self.base.execute_api_call("/tag/getInfo", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if args.positional.is_empty() && !args.named.contains_key("tag") {
            return Err(CliError::missing_argument("tag"));
        }
        Ok(())
    }
}

/// Get top artists for a tag
pub struct TagTopArtistsCommand {
    base: BaseCommand,
}

impl TagTopArtistsCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "tag.top-artists",
                "Get top artists for a tag",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for TagTopArtistsCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        let tag = get_required_arg(args, "tag")?;
        params.insert("tag".to_string(), tag);
        
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        
        self.base.execute_api_call("/tag/getTopArtists", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if args.positional.is_empty() && !args.named.contains_key("tag") {
            return Err(CliError::missing_argument("tag"));
        }
        Ok(())
    }
}

/// Get top albums for a tag
pub struct TagTopAlbumsCommand {
    base: BaseCommand,
}

impl TagTopAlbumsCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "tag.top-albums",
                "Get top albums for a tag",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for TagTopAlbumsCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        let tag = get_required_arg(args, "tag")?;
        params.insert("tag".to_string(), tag);
        
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        
        self.base.execute_api_call("/tag/getTopAlbums", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if args.positional.is_empty() && !args.named.contains_key("tag") {
            return Err(CliError::missing_argument("tag"));
        }
        Ok(())
    }
}

/// Get top tracks for a tag
pub struct TagTopTracksCommand {
    base: BaseCommand,
}

impl TagTopTracksCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "tag.top-tracks",
                "Get top tracks for a tag",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for TagTopTracksCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        let tag = get_required_arg(args, "tag")?;
        params.insert("tag".to_string(), tag);
        
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        
        self.base.execute_api_call("/tag/getTopTracks", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if args.positional.is_empty() && !args.named.contains_key("tag") {
            return Err(CliError::missing_argument("tag"));
        }
        Ok(())
    }
}