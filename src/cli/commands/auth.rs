// Authentication commands

use async_trait::async_trait;
use std::sync::Arc;

use crate::cli::{
    api::LastfmApiClient,
    auth::{AuthManager, AuthStatus},
    config::ConfigManager,
    error::Result,
    traits::{ApiClient, Command, CommandArgs, CommandOutput, OutputMetadata},
};

use super::BaseCommand;

/// Auth login command
pub struct AuthLoginCommand {
    base: BaseCommand,
    config_manager: ConfigManager,
}

impl AuthLoginCommand {
    pub fn new(api_client: Arc<dyn ApiClient>, config_manager: ConfigManager) -> Self {
        Self {
            base: BaseCommand::new("auth.login", "Authenticate with Last.fm", api_client),
            config_manager,
        }
    }
}

#[async_trait]
impl Command for AuthLoginCommand {
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
        // Get API client
        let api_client = match self
            .base
            .api_client
            .as_any()
            .downcast_ref::<LastfmApiClient>()
        {
            Some(client) => client.clone(),
            None => {
                return Err(crate::cli::error::CliError::other(
                    "Invalid API client type",
                ))
            }
        };

        // Create auth manager
        let auth_manager = AuthManager::new(api_client, self.config_manager.clone());

        // Start login flow
        let session = auth_manager.login().await?;

        Ok(CommandOutput {
            data: serde_json::json!({
                "authenticated": true,
                "username": session.username,
                "message": format!("Successfully authenticated as '{}'", session.username)
            }),
            metadata: OutputMetadata::default(),
        })
    }
}

/// Auth status command
pub struct AuthStatusCommand {
    base: BaseCommand,
    config_manager: ConfigManager,
}

impl AuthStatusCommand {
    pub fn new(api_client: Arc<dyn ApiClient>, config_manager: ConfigManager) -> Self {
        Self {
            base: BaseCommand::new("auth.status", "Check authentication status", api_client),
            config_manager,
        }
    }
}

#[async_trait]
impl Command for AuthStatusCommand {
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
        // Get API client
        let api_client = match self
            .base
            .api_client
            .as_any()
            .downcast_ref::<LastfmApiClient>()
        {
            Some(client) => client.clone(),
            None => {
                return Err(crate::cli::error::CliError::other(
                    "Invalid API client type",
                ))
            }
        };

        // Create auth manager
        let auth_manager = AuthManager::new(api_client, self.config_manager.clone());

        // Get session
        let session = auth_manager.get_session().await?;
        let status = AuthStatus::new(session);

        Ok(CommandOutput {
            data: serde_json::to_value(&status)?,
            metadata: OutputMetadata::default(),
        })
    }
}

/// Auth logout command
pub struct AuthLogoutCommand {
    base: BaseCommand,
    config_manager: ConfigManager,
}

impl AuthLogoutCommand {
    pub fn new(api_client: Arc<dyn ApiClient>, config_manager: ConfigManager) -> Self {
        Self {
            base: BaseCommand::new("auth.logout", "Log out and clear session", api_client),
            config_manager,
        }
    }
}

#[async_trait]
impl Command for AuthLogoutCommand {
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
        // Get API client
        let api_client = match self
            .base
            .api_client
            .as_any()
            .downcast_ref::<LastfmApiClient>()
        {
            Some(client) => client.clone(),
            None => {
                return Err(crate::cli::error::CliError::other(
                    "Invalid API client type",
                ))
            }
        };

        // Create auth manager
        let auth_manager = AuthManager::new(api_client, self.config_manager.clone());

        // Logout
        auth_manager.logout().await?;

        Ok(CommandOutput {
            data: serde_json::json!({
                "authenticated": false,
                "message": "Successfully logged out"
            }),
            metadata: OutputMetadata::default(),
        })
    }
}
