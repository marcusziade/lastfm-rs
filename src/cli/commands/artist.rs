// Artist command implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::cli::{
    error::{CliError, Result},
    traits::{ApiClient, Command, CommandArgs, CommandOutput},
};

use super::{get_flag, get_optional_arg, get_required_arg, BaseCommand};

/// Get artist information
pub struct ArtistInfoCommand {
    base: BaseCommand,
}

impl ArtistInfoCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "artist.info",
                "Get artist metadata including biography",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for ArtistInfoCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();

        // Get artist name or mbid
        if let Ok(artist) = get_required_arg(args, "artist") {
            params.insert("artist".to_string(), artist);
        } else if let Some(mbid) = args.named.get("mbid") {
            params.insert("mbid".to_string(), mbid.clone());
        } else {
            return Err(CliError::validation(
                "Either artist name or mbid is required",
            ));
        }

        // Optional parameters
        if get_flag(args, "autocorrect") {
            params.insert("autocorrect".to_string(), "1".to_string());
        }

        if let Some(lang) = args.named.get("lang") {
            params.insert("lang".to_string(), lang.clone());
        }

        if let Some(username) = args.named.get("username") {
            params.insert("username".to_string(), username.clone());
        }

        self.base.execute_api_call("/artist/getInfo", params).await
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        // Check that either artist or mbid is provided
        if !args.named.contains_key("artist")
            && !args.named.contains_key("mbid")
            && args.positional.is_empty()
        {
            return Err(CliError::validation(
                "Either artist name or mbid is required",
            ));
        }
        Ok(())
    }
}

/// Get similar artists
pub struct ArtistSimilarCommand {
    base: BaseCommand,
}

impl ArtistSimilarCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new("artist.similar", "Get similar artists", api_client),
        }
    }
}

#[async_trait]
impl Command for ArtistSimilarCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();

        // Get artist name or mbid
        if let Ok(artist) = get_required_arg(args, "artist") {
            params.insert("artist".to_string(), artist);
        } else if let Some(mbid) = args.named.get("mbid") {
            params.insert("mbid".to_string(), mbid.clone());
        } else {
            return Err(CliError::validation(
                "Either artist name or mbid is required",
            ));
        }

        // Optional parameters
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );

        if get_flag(args, "autocorrect") {
            params.insert("autocorrect".to_string(), "1".to_string());
        }

        self.base
            .execute_api_call("/artist/getSimilar", params)
            .await
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if !args.named.contains_key("artist")
            && !args.named.contains_key("mbid")
            && args.positional.is_empty()
        {
            return Err(CliError::validation(
                "Either artist name or mbid is required",
            ));
        }
        Ok(())
    }
}

/// Get top albums for an artist
pub struct ArtistTopAlbumsCommand {
    base: BaseCommand,
}

impl ArtistTopAlbumsCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "artist.top-albums",
                "Get top albums for an artist",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for ArtistTopAlbumsCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();

        // Get artist name or mbid
        if let Ok(artist) = get_required_arg(args, "artist") {
            params.insert("artist".to_string(), artist);
        } else if let Some(mbid) = args.named.get("mbid") {
            params.insert("mbid".to_string(), mbid.clone());
        } else {
            return Err(CliError::validation(
                "Either artist name or mbid is required",
            ));
        }

        // Pagination
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );

        if get_flag(args, "autocorrect") {
            params.insert("autocorrect".to_string(), "1".to_string());
        }

        self.base
            .execute_api_call("/artist/getTopAlbums", params)
            .await
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if !args.named.contains_key("artist")
            && !args.named.contains_key("mbid")
            && args.positional.is_empty()
        {
            return Err(CliError::validation(
                "Either artist name or mbid is required",
            ));
        }
        Ok(())
    }
}

/// Get top tracks for an artist
pub struct ArtistTopTracksCommand {
    base: BaseCommand,
}

impl ArtistTopTracksCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "artist.top-tracks",
                "Get top tracks by an artist",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for ArtistTopTracksCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();

        // Get artist name or mbid
        if let Ok(artist) = get_required_arg(args, "artist") {
            params.insert("artist".to_string(), artist);
        } else if let Some(mbid) = args.named.get("mbid") {
            params.insert("mbid".to_string(), mbid.clone());
        } else {
            return Err(CliError::validation(
                "Either artist name or mbid is required",
            ));
        }

        // Pagination
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );

        if get_flag(args, "autocorrect") {
            params.insert("autocorrect".to_string(), "1".to_string());
        }

        self.base
            .execute_api_call("/artist/getTopTracks", params)
            .await
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if !args.named.contains_key("artist")
            && !args.named.contains_key("mbid")
            && args.positional.is_empty()
        {
            return Err(CliError::validation(
                "Either artist name or mbid is required",
            ));
        }
        Ok(())
    }
}

/// Search for artists
pub struct ArtistSearchCommand {
    base: BaseCommand,
}

impl ArtistSearchCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new("artist.search", "Search for artists", api_client),
        }
    }
}

#[async_trait]
impl Command for ArtistSearchCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();

        // Get search query
        let query = get_required_arg(args, "query")?;
        params.insert("artist".to_string(), query);

        // Pagination
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("30")),
        );

        self.base.execute_api_call("/artist/search", params).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist_info_validation() {
        let api_client = Arc::new(crate::cli::api::LastfmApiClient::new(
            "http://test.com".to_string(),
        ));
        let cmd = ArtistInfoCommand::new(api_client);

        // Test with no arguments
        let args = CommandArgs::default();
        assert!(cmd.validate_args(&args).is_err());

        // Test with artist name
        let mut args = CommandArgs::default();
        args.named
            .insert("artist".to_string(), "The Beatles".to_string());
        assert!(cmd.validate_args(&args).is_ok());

        // Test with mbid
        let mut args = CommandArgs::default();
        args.named
            .insert("mbid".to_string(), "test-mbid".to_string());
        assert!(cmd.validate_args(&args).is_ok());
    }
}
