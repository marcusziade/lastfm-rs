// Cache management command implementations

use async_trait::async_trait;
use crate::cli::{
    error::Result,
    traits::{Command, CommandArgs, CommandOutput, OutputMetadata, CacheManager},
};

/// Clear cache command
pub struct ClearCacheCommand<C: CacheManager> {
    cache_manager: C,
}

impl<C: CacheManager> ClearCacheCommand<C> {
    pub fn new(cache_manager: C) -> Self {
        Self { cache_manager }
    }
}

#[async_trait]
impl<C: CacheManager + Send + Sync> Command for ClearCacheCommand<C> {
    async fn execute(&self, _args: &CommandArgs) -> Result<CommandOutput> {
        self.cache_manager.clear().await?;
        
        let data = serde_json::json!({
            "status": "success",
            "message": "Cache cleared successfully"
        });
        
        Ok(CommandOutput {
            data,
            metadata: OutputMetadata::default(),
        })
    }
    
    fn name(&self) -> &str {
        "cache.clear"
    }
    
    fn description(&self) -> &str {
        "Clear all cached entries"
    }
    
    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
}