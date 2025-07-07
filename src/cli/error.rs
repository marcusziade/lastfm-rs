// Error handling for the CLI

use thiserror::Error;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Cache error: {0}")]
    Cache(String),
    
    #[error("Worker error: {0}")]
    Worker(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Authentication required")]
    AuthRequired,
    
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    
    #[error("Missing argument: {0}")]
    MissingArgument(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("{0}")]
    Other(String),
}

impl CliError {
    /// Create an API error with a custom message
    pub fn api(msg: impl Into<String>) -> Self {
        Self::Api(msg.into())
    }
    
    /// Create a configuration error with a custom message
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }
    
    /// Create a validation error with a custom message
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
    
    /// Create a cache error with a custom message
    pub fn cache(msg: impl Into<String>) -> Self {
        Self::Cache(msg.into())
    }
    
    /// Create a worker error with a custom message
    pub fn worker(msg: impl Into<String>) -> Self {
        Self::Worker(msg.into())
    }
    
    /// Create a not found error with a custom message
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
    
    /// Create an invalid command error
    pub fn invalid_command(msg: impl Into<String>) -> Self {
        Self::InvalidCommand(msg.into())
    }
    
    /// Create a missing argument error
    pub fn missing_argument(arg: impl Into<String>) -> Self {
        Self::MissingArgument(arg.into())
    }
    
    /// Create an invalid argument error
    pub fn invalid_argument(msg: impl Into<String>) -> Self {
        Self::InvalidArgument(msg.into())
    }
    
    /// Create a generic error with a custom message
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
    
    /// Check if this is a retryable error
    pub fn is_retryable(&self) -> bool {
        matches!(self, 
            Self::Http(_) | 
            Self::RateLimit | 
            Self::Api(_) |
            Self::Cache(_)
        )
    }
    
    /// Get the error code for API responses
    pub fn error_code(&self) -> u32 {
        match self {
            Self::RateLimit => 429,
            Self::AuthRequired => 401,
            Self::NotFound(_) => 404,
            Self::Validation(_) | Self::InvalidArgument(_) | Self::MissingArgument(_) => 400,
            _ => 500,
        }
    }
}