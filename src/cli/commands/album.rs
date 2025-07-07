// Album command implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::cli::{
    error::{CliError, Result},
    traits::{ApiClient, Command, CommandArgs, CommandOutput},
};

use super::{get_flag, get_optional_arg, get_required_arg, BaseCommand};

/// Get album information
pub struct AlbumInfoCommand {
    base: BaseCommand,
}

impl AlbumInfoCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new("album.info", "Get album metadata and tracklist", api_client),
        }
    }
}

#[async_trait]
impl Command for AlbumInfoCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();

        // Get artist and album or mbid
        if let Some(mbid) = args.named.get("mbid") {
            params.insert("mbid".to_string(), mbid.clone());
        } else {
            let artist = get_required_arg(args, "artist")?;
            let album = args
                .named
                .get("album")
                .cloned()
                .or_else(|| args.positional.get(1).cloned())
                .ok_or_else(|| CliError::missing_argument("album"))?;

            params.insert("artist".to_string(), artist);
            params.insert("album".to_string(), album);
        }

        // Optional parameters
        if get_flag(args, "autocorrect") {
            params.insert("autocorrect".to_string(), "1".to_string());
        }

        if let Some(username) = args.named.get("username") {
            params.insert("username".to_string(), username.clone());
        }

        if let Some(lang) = args.named.get("lang") {
            params.insert("lang".to_string(), lang.clone());
        }

        self.base.execute_api_call("/album/getInfo", params).await
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if !args.named.contains_key("mbid") {
            // Need both artist and album
            if args.positional.len() < 2
                && (!args.named.contains_key("artist") || !args.named.contains_key("album"))
            {
                return Err(CliError::validation(
                    "Either mbid or both artist and album are required",
                ));
            }
        }
        Ok(())
    }
}

/// Search for albums
pub struct AlbumSearchCommand {
    base: BaseCommand,
}

impl AlbumSearchCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new("album.search", "Search for albums", api_client),
        }
    }
}

#[async_trait]
impl Command for AlbumSearchCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();

        // Get search query
        let query = get_required_arg(args, "query")?;
        params.insert("album".to_string(), query);

        // Pagination
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("30")),
        );

        self.base.execute_api_call("/album/search", params).await
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
