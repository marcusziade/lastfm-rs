// Trait definitions (similar to Swift protocols)

use crate::cli::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;

/// Trait for API clients - defines how to interact with APIs
#[async_trait]
pub trait ApiClient: Send + Sync {
    /// Make a GET request to the API
    async fn get(
        &self,
        endpoint: &str,
        params: &HashMap<String, String>,
    ) -> Result<serde_json::Value>;

    /// Make a POST request to the API
    async fn post(&self, endpoint: &str, body: &serde_json::Value) -> Result<serde_json::Value>;

    /// Get the base URL for the API
    fn base_url(&self) -> &str;

    /// Check if the API is reachable
    async fn health_check(&self) -> Result<bool>;

    /// Get self as Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Trait for executable commands
#[async_trait]
pub trait Command: Send + Sync {
    /// Execute the command with given arguments
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput>;

    /// Get the command name
    fn name(&self) -> &str;

    /// Get the command description
    fn description(&self) -> &str;

    /// Validate arguments before execution
    fn validate_args(&self, args: &CommandArgs) -> Result<()>;
}

/// Command arguments container
#[derive(Debug, Clone, Default)]
pub struct CommandArgs {
    pub positional: Vec<String>,
    pub named: HashMap<String, String>,
    pub flags: HashMap<String, bool>,
}

/// Command output container
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub data: serde_json::Value,
    pub metadata: OutputMetadata,
}

/// Metadata about the command output
#[derive(Debug, Clone, Default)]
pub struct OutputMetadata {
    pub cache_hit: bool,
    pub response_time_ms: u64,
    pub api_calls_made: u32,
}

/// Trait for output formatting
pub trait OutputFormatter {
    /// Format the output for display
    fn format(&self, output: &CommandOutput) -> String;

    /// Get the formatter name
    fn name(&self) -> &str;
}

/// Trait for configuration management
#[async_trait]
pub trait Configurable {
    type Config: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync;

    /// Load configuration from storage
    async fn load(&self) -> Result<Self::Config>;

    /// Save configuration to storage
    async fn save(&self, config: &Self::Config) -> Result<()>;

    /// Get default configuration
    fn default_config(&self) -> Self::Config;

    /// Validate configuration
    fn validate(&self, config: &Self::Config) -> Result<()>;
}

/// Trait for cache management
#[async_trait]
pub trait CacheManager: Send + Sync {
    /// Get a value from cache
    async fn get(&self, key: &str) -> Result<Option<String>>;

    /// Set a value in cache with TTL
    async fn set(&self, key: &str, value: &str, ttl_seconds: u64) -> Result<()>;

    /// Delete a value from cache
    async fn delete(&self, key: &str) -> Result<()>;

    /// Clear all cache entries
    async fn clear(&self) -> Result<()>;

    /// Get cache statistics
    async fn stats(&self) -> Result<CacheStats>;
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_keys: u64,
    pub total_size_bytes: u64,
    pub hit_rate: f32,
    pub miss_rate: f32,
}

/// Trait for worker management
#[async_trait]
pub trait WorkerManager: Send + Sync {
    /// Deploy the worker
    async fn deploy(&self, env: Option<&str>) -> Result<DeploymentInfo>;

    /// Get worker logs
    async fn logs(&self, tail: bool) -> Result<Vec<LogEntry>>;

    /// Get worker status
    async fn status(&self) -> Result<WorkerStatus>;
}

/// Deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentInfo {
    pub worker_name: String,
    pub deployment_id: String,
    pub url: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: String,
    pub message: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Worker status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStatus {
    pub is_deployed: bool,
    pub last_deployment: Option<chrono::DateTime<chrono::Utc>>,
    pub requests_24h: u64,
    pub errors_24h: u64,
    pub cpu_time_ms: u64,
}

/// Trait for interactive mode handlers
#[async_trait]
pub trait InteractiveHandler: Send + Sync {
    /// Handle a command in interactive mode
    async fn handle(&self, input: &str) -> Result<String>;

    /// Get command suggestions for autocomplete
    fn suggestions(&self, partial: &str) -> Vec<String>;

    /// Get the prompt string
    fn prompt(&self) -> &str;
}
