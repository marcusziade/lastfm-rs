// CLI module - main entry point for the CLI architecture

pub mod api;
pub mod auth;
pub mod commands;
pub mod config;
pub mod error;
pub mod output;
pub mod traits;

// Re-export commonly used types
pub use api::LastfmApiClient;
pub use config::ConfigManager;
pub use error::{CliError, Result};
pub use output::{create_formatter, OutputFormat};
pub use traits::{ApiClient, Command, Configurable};