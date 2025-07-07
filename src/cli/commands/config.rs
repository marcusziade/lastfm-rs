// Configuration command implementations

use async_trait::async_trait;
use crate::cli::{
    config::{CliConfig, ConfigField},
    error::{CliError, Result},
    traits::{Command, CommandArgs, CommandOutput, OutputMetadata, Configurable},
};

use super::get_required_arg;

/// Set configuration value command
pub struct SetConfigCommand<C: Configurable<Config = CliConfig>> {
    config_manager: C,
}

impl<C: Configurable<Config = CliConfig>> SetConfigCommand<C> {
    pub fn new(config_manager: C) -> Self {
        Self { config_manager }
    }
}

#[async_trait]
impl<C: Configurable<Config = CliConfig> + Send + Sync> Command for SetConfigCommand<C> {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let key = get_required_arg(args, "key")?;
        let value = get_required_arg(args, "value")?;
        
        let field = ConfigField::from_str(&key)
            .ok_or_else(|| CliError::invalid_argument(format!("Unknown config key: {}", key)))?;
        
        let mut config = self.config_manager.load().await?;
        config.set_value(field, &value)?;
        
        self.config_manager.validate(&config)?;
        self.config_manager.save(&config).await?;
        
        let data = serde_json::json!({
            "status": "success",
            "key": key,
            "value": value
        });
        
        Ok(CommandOutput {
            data,
            metadata: OutputMetadata::default(),
        })
    }
    
    fn name(&self) -> &str {
        "config.set"
    }
    
    fn description(&self) -> &str {
        "Set a configuration value"
    }
    
    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if !args.named.contains_key("key") || !args.named.contains_key("value") {
            return Err(CliError::validation("Both key and value are required"));
        }
        Ok(())
    }
}