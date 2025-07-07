// Chart command implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::cli::{
    error::Result,
    traits::{ApiClient, Command, CommandArgs, CommandOutput},
};

use super::{BaseCommand, get_optional_arg};

/// Get top artists chart
pub struct ChartTopArtistsCommand {
    base: BaseCommand,
}

impl ChartTopArtistsCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "chart.top-artists",
                "Get top artists chart",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for ChartTopArtistsCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        
        self.base.execute_api_call("/chart/getTopArtists", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
}

/// Get top tracks chart
pub struct ChartTopTracksCommand {
    base: BaseCommand,
}

impl ChartTopTracksCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "chart.top-tracks",
                "Get top tracks chart",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for ChartTopTracksCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        
        self.base.execute_api_call("/chart/getTopTracks", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
}

/// Get top tags chart
pub struct ChartTopTagsCommand {
    base: BaseCommand,
}

impl ChartTopTagsCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "chart.top-tags",
                "Get top tags chart",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for ChartTopTagsCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        
        self.base.execute_api_call("/chart/getTopTags", params).await
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
}