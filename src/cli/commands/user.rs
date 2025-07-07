// User command implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::cli::{
    error::{CliError, Result},
    traits::{ApiClient, Command, CommandArgs, CommandOutput},
};

use super::{get_flag, get_optional_arg, get_required_arg, BaseCommand};

/// Get user information
pub struct UserInfoCommand {
    base: BaseCommand,
}

impl UserInfoCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new("user.info", "Get user profile information", api_client),
        }
    }
}

#[async_trait]
impl Command for UserInfoCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();

        if let Ok(user) = get_required_arg(args, "user") {
            params.insert("user".to_string(), user);
        }

        self.base.execute_api_call("/user/getInfo", params).await
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        // User parameter is optional for this endpoint
        Ok(())
    }
}

/// Get user's top artists
pub struct UserTopArtistsCommand {
    base: BaseCommand,
}

impl UserTopArtistsCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new("user.top-artists", "Get user's top artists", api_client),
        }
    }
}

#[async_trait]
impl Command for UserTopArtistsCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();

        let user = get_required_arg(args, "user")?;
        params.insert("user".to_string(), user);

        params.insert(
            "period".to_string(),
            get_optional_arg(args, "period", Some("overall")),
        );
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );

        self.base
            .execute_api_call("/user/getTopArtists", params)
            .await
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if args.positional.is_empty() && !args.named.contains_key("user") {
            return Err(CliError::missing_argument("user"));
        }
        Ok(())
    }
}

/// Get user's recent tracks
pub struct UserRecentTracksCommand {
    base: BaseCommand,
}

impl UserRecentTracksCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new("user.recent-tracks", "Get user's recent tracks", api_client),
        }
    }
}

#[async_trait]
impl Command for UserRecentTracksCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();

        let user = get_required_arg(args, "user")?;
        params.insert("user".to_string(), user);

        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );

        if get_flag(args, "extended") {
            params.insert("extended".to_string(), "1".to_string());
        }

        if let Some(from) = args.named.get("from") {
            params.insert("from".to_string(), from.clone());
        }

        if let Some(to) = args.named.get("to") {
            params.insert("to".to_string(), to.clone());
        }

        self.base
            .execute_api_call("/user/getRecentTracks", params)
            .await
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if args.positional.is_empty() && !args.named.contains_key("user") {
            return Err(CliError::missing_argument("user"));
        }
        Ok(())
    }
}
