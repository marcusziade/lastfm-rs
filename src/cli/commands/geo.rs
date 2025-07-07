// Geographic command implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::cli::{
    error::{CliError, Result},
    traits::{ApiClient, Command, CommandArgs, CommandOutput},
};

use super::{BaseCommand, get_required_arg, get_optional_arg};

/// Get top artists by country
pub struct GeoTopArtistsCommand {
    base: BaseCommand,
}

impl GeoTopArtistsCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "geo.top-artists",
                "Get top artists by country",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for GeoTopArtistsCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        let country = get_required_arg(args, "country")?;
        params.insert("country".to_string(), country);
        
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        
        self.base.execute_api_call("/geo/getTopArtists", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if args.positional.is_empty() && !args.named.contains_key("country") {
            return Err(CliError::missing_argument("country"));
        }
        Ok(())
    }
}

/// Get top tracks by country
pub struct GeoTopTracksCommand {
    base: BaseCommand,
}

impl GeoTopTracksCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "geo.top-tracks",
                "Get top tracks by country",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for GeoTopTracksCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        let country = get_required_arg(args, "country")?;
        params.insert("country".to_string(), country);
        
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        
        self.base.execute_api_call("/geo/getTopTracks", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if args.positional.is_empty() && !args.named.contains_key("country") {
            return Err(CliError::missing_argument("country"));
        }
        Ok(())
    }
}