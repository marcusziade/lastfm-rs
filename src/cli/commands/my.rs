// "My" commands for authenticated users

use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;

use crate::cli::{
    auth::AuthManager,
    config::ConfigManager,
    error::{CliError, Result},
    traits::{ApiClient, Command, CommandArgs, CommandOutput},
};

use super::{BaseCommand, get_optional_arg, get_flag};

/// My recent tracks command
pub struct MyRecentTracksCommand {
    base: BaseCommand,
    config_manager: ConfigManager,
}

impl MyRecentTracksCommand {
    pub fn new(api_client: Arc<dyn ApiClient>, config_manager: ConfigManager) -> Self {
        Self {
            base: BaseCommand::new(
                "my.recent-tracks",
                "Get your recent tracks (requires authentication)",
                api_client,
            ),
            config_manager,
        }
    }
}

#[async_trait]
impl Command for MyRecentTracksCommand {
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
    
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        // Get authenticated user
        let auth_manager = AuthManager::new(
            self.base.api_client.as_any()
                .downcast_ref::<crate::cli::api::LastfmApiClient>()
                .ok_or_else(|| CliError::other("Invalid API client type"))?
                .clone(),
            self.config_manager.clone()
        );
        
        let session = auth_manager.get_session().await?
            .ok_or_else(|| CliError::other("Not authenticated. Use 'lastfm-cli auth login' first"))?;
        
        // Build parameters
        let mut params = HashMap::new();
        params.insert("user".to_string(), session.username);
        
        if get_flag(args, "extended") {
            params.insert("extended".to_string(), "1".to_string());
        }
        
        let limit = get_optional_arg(args, "limit", Some("50"));
        if !limit.is_empty() {
            params.insert("limit".to_string(), limit);
        }
        
        let page = get_optional_arg(args, "page", Some("1"));
        if !page.is_empty() {
            params.insert("page".to_string(), page);
        }
        
        self.base.execute_api_call("/user/getRecentTracks", params).await
    }
}

/// My top artists command
pub struct MyTopArtistsCommand {
    base: BaseCommand,
    config_manager: ConfigManager,
}

impl MyTopArtistsCommand {
    pub fn new(api_client: Arc<dyn ApiClient>, config_manager: ConfigManager) -> Self {
        Self {
            base: BaseCommand::new(
                "my.top-artists",
                "Get your top artists (requires authentication)",
                api_client,
            ),
            config_manager,
        }
    }
}

#[async_trait]
impl Command for MyTopArtistsCommand {
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
    
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        // Get authenticated user
        let auth_manager = AuthManager::new(
            self.base.api_client.as_any()
                .downcast_ref::<crate::cli::api::LastfmApiClient>()
                .ok_or_else(|| CliError::other("Invalid API client type"))?
                .clone(),
            self.config_manager.clone()
        );
        
        let session = auth_manager.get_session().await?
            .ok_or_else(|| CliError::other("Not authenticated. Use 'lastfm-cli auth login' first"))?;
        
        // Build parameters
        let mut params = HashMap::new();
        params.insert("user".to_string(), session.username);
        
        let period = get_optional_arg(args, "period", Some("overall"));
        if !period.is_empty() {
            params.insert("period".to_string(), period);
        }
        
        let limit = get_optional_arg(args, "limit", Some("50"));
        if !limit.is_empty() {
            params.insert("limit".to_string(), limit);
        }
        
        let page = get_optional_arg(args, "page", Some("1"));
        if !page.is_empty() {
            params.insert("page".to_string(), page);
        }
        
        self.base.execute_api_call("/user/getTopArtists", params).await
    }
}

/// My top tracks command
pub struct MyTopTracksCommand {
    base: BaseCommand,
    config_manager: ConfigManager,
}

impl MyTopTracksCommand {
    pub fn new(api_client: Arc<dyn ApiClient>, config_manager: ConfigManager) -> Self {
        Self {
            base: BaseCommand::new(
                "my.top-tracks",
                "Get your top tracks (requires authentication)",
                api_client,
            ),
            config_manager,
        }
    }
}

#[async_trait]
impl Command for MyTopTracksCommand {
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
    
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        // Get authenticated user
        let auth_manager = AuthManager::new(
            self.base.api_client.as_any()
                .downcast_ref::<crate::cli::api::LastfmApiClient>()
                .ok_or_else(|| CliError::other("Invalid API client type"))?
                .clone(),
            self.config_manager.clone()
        );
        
        let session = auth_manager.get_session().await?
            .ok_or_else(|| CliError::other("Not authenticated. Use 'lastfm-cli auth login' first"))?;
        
        // Build parameters
        let mut params = HashMap::new();
        params.insert("user".to_string(), session.username);
        
        let period = get_optional_arg(args, "period", Some("overall"));
        if !period.is_empty() {
            params.insert("period".to_string(), period);
        }
        
        let limit = get_optional_arg(args, "limit", Some("50"));
        if !limit.is_empty() {
            params.insert("limit".to_string(), limit);
        }
        
        let page = get_optional_arg(args, "page", Some("1"));
        if !page.is_empty() {
            params.insert("page".to_string(), page);
        }
        
        self.base.execute_api_call("/user/getTopTracks", params).await
    }
}

/// My top albums command
pub struct MyTopAlbumsCommand {
    base: BaseCommand,
    config_manager: ConfigManager,
}

impl MyTopAlbumsCommand {
    pub fn new(api_client: Arc<dyn ApiClient>, config_manager: ConfigManager) -> Self {
        Self {
            base: BaseCommand::new(
                "my.top-albums",
                "Get your top albums (requires authentication)",
                api_client,
            ),
            config_manager,
        }
    }
}

#[async_trait]
impl Command for MyTopAlbumsCommand {
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
    
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        // Get authenticated user
        let auth_manager = AuthManager::new(
            self.base.api_client.as_any()
                .downcast_ref::<crate::cli::api::LastfmApiClient>()
                .ok_or_else(|| CliError::other("Invalid API client type"))?
                .clone(),
            self.config_manager.clone()
        );
        
        let session = auth_manager.get_session().await?
            .ok_or_else(|| CliError::other("Not authenticated. Use 'lastfm-cli auth login' first"))?;
        
        // Build parameters
        let mut params = HashMap::new();
        params.insert("user".to_string(), session.username);
        
        let period = get_optional_arg(args, "period", Some("overall"));
        if !period.is_empty() {
            params.insert("period".to_string(), period);
        }
        
        let limit = get_optional_arg(args, "limit", Some("50"));
        if !limit.is_empty() {
            params.insert("limit".to_string(), limit);
        }
        
        let page = get_optional_arg(args, "page", Some("1"));
        if !page.is_empty() {
            params.insert("page".to_string(), page);
        }
        
        self.base.execute_api_call("/user/getTopAlbums", params).await
    }
}

/// My info command
pub struct MyInfoCommand {
    base: BaseCommand,
    config_manager: ConfigManager,
}

impl MyInfoCommand {
    pub fn new(api_client: Arc<dyn ApiClient>, config_manager: ConfigManager) -> Self {
        Self {
            base: BaseCommand::new(
                "my.info",
                "Get your user information (requires authentication)",
                api_client,
            ),
            config_manager,
        }
    }
}

#[async_trait]
impl Command for MyInfoCommand {
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn description(&self) -> &str {
        &self.base.description
    }
    
    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
    
    async fn execute(&self, _args: &CommandArgs) -> Result<CommandOutput> {
        // Get authenticated user
        let auth_manager = AuthManager::new(
            self.base.api_client.as_any()
                .downcast_ref::<crate::cli::api::LastfmApiClient>()
                .ok_or_else(|| CliError::other("Invalid API client type"))?
                .clone(),
            self.config_manager.clone()
        );
        
        let session = auth_manager.get_session().await?
            .ok_or_else(|| CliError::other("Not authenticated. Use 'lastfm-cli auth login' first"))?;
        
        // Build parameters
        let mut params = HashMap::new();
        params.insert("user".to_string(), session.username);
        
        self.base.execute_api_call("/user/getInfo", params).await
    }
}