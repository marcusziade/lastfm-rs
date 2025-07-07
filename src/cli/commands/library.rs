// Library command implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::cli::{
    error::{CliError, Result},
    traits::{ApiClient, Command, CommandArgs, CommandOutput},
};

use super::{BaseCommand, get_required_arg, get_optional_arg};

/// Get all artists in a user's library
pub struct LibraryArtistsCommand {
    base: BaseCommand,
}

impl LibraryArtistsCommand {
    pub fn new(api_client: Arc<dyn ApiClient>) -> Self {
        Self {
            base: BaseCommand::new(
                "library.artists",
                "Get all artists in a user's library",
                api_client,
            ),
        }
    }
}

#[async_trait]
impl Command for LibraryArtistsCommand {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let mut params = HashMap::new();
        
        let user = get_required_arg(args, "user")?;
        params.insert("user".to_string(), user);
        
        params.insert(
            "page".to_string(),
            get_optional_arg(args, "page", Some("1")),
        );
        params.insert(
            "limit".to_string(),
            get_optional_arg(args, "limit", Some("50")),
        );
        
        self.base.execute_api_call("/library/getArtists", params).await
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