// Authentication module for Last.fm CLI

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cli::{
    api::LastfmApiClient,
    config::ConfigManager,
    error::{CliError, Result},
    traits::{ApiClient, Configurable},
};

/// Authentication manager for Last.fm
pub struct AuthManager {
    api_client: LastfmApiClient,
    config_manager: ConfigManager,
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub username: String,
    pub key: String,
}

/// Auth configuration stored in config file
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    pub username: Option<String>,
    pub session_key: Option<String>,
}

impl AuthManager {
    /// Create a new auth manager
    pub fn new(api_client: LastfmApiClient, config_manager: ConfigManager) -> Self {
        Self {
            api_client,
            config_manager,
        }
    }

    /// Get the current session if available
    pub async fn get_session(&self) -> Result<Option<Session>> {
        let config = self.config_manager.load().await?;

        match (config.auth.username, config.auth.session_key) {
            (Some(username), Some(key)) => Ok(Some(Session { username, key })),
            _ => Ok(None),
        }
    }

    /// Check if user is authenticated
    pub async fn is_authenticated(&self) -> Result<bool> {
        Ok(self.get_session().await?.is_some())
    }

    /// Generate auth URL for user to authorize the application
    pub async fn generate_auth_url(&self) -> Result<String> {
        // Get the auth URL from the worker which has the API key
        let api_client: &dyn ApiClient = &self.api_client;
        let data = api_client.get("/auth/url", &HashMap::new()).await?;

        let auth_url = data
            .get("auth_url")
            .and_then(|u| u.as_str())
            .ok_or_else(|| CliError::api("Invalid response: missing auth_url"))?
            .to_string();

        Ok(auth_url)
    }

    /// Get session from auth token
    pub async fn get_session_from_token(&self, token: &str) -> Result<Session> {
        let mut params = HashMap::new();
        params.insert("token".to_string(), token.to_string());

        // Make the request through the worker endpoint which will handle the API key
        let api_client: &dyn ApiClient = &self.api_client;
        let data = api_client.get("/auth/getSession", &params).await?;

        if let Some(_error) = data.get("error") {
            let message = data
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Authentication failed");
            return Err(CliError::api(message));
        }

        let session = data
            .get("session")
            .ok_or_else(|| CliError::api("Invalid response: missing session"))?;

        let username = session
            .get("name")
            .and_then(|n| n.as_str())
            .ok_or_else(|| CliError::api("Invalid response: missing username"))?
            .to_string();

        let key = session
            .get("key")
            .and_then(|k| k.as_str())
            .ok_or_else(|| CliError::api("Invalid response: missing session key"))?
            .to_string();

        Ok(Session { username, key })
    }

    /// Start the authentication flow
    pub async fn login(&self) -> Result<Session> {
        // Generate auth URL from worker
        let auth_url = self.generate_auth_url().await?;

        println!("Opening browser for authorization...");
        println!("If the browser doesn't open, visit this URL:");
        println!("{auth_url}");

        // Try to open browser
        let _ = open::that(&auth_url);

        println!("\nAfter authorizing, you'll be redirected to a page showing an auth token.");
        println!("Please enter the token here:");

        // Read token from user using dialoguer
        use dialoguer::Input;
        let token: String = Input::new()
            .with_prompt("Token")
            .interact_text()
            .map_err(|e| CliError::other(format!("Failed to read input: {e}")))?;

        // Get session from token
        let session = self.get_session_from_token(&token).await?;

        // Save session to config
        self.save_session(&session).await?;

        println!("✓ Successfully authenticated as '{}'", session.username);

        Ok(session)
    }

    /// Save session to config
    async fn save_session(&self, session: &Session) -> Result<()> {
        let mut config = self.config_manager.load().await?;
        config.auth.username = Some(session.username.clone());
        config.auth.session_key = Some(session.key.clone());
        self.config_manager.save(&config).await?;
        Ok(())
    }

    /// Logout and clear session
    pub async fn logout(&self) -> Result<()> {
        let mut config = self.config_manager.load().await?;
        config.auth.username = None;
        config.auth.session_key = None;
        self.config_manager.save(&config).await?;
        println!("Successfully logged out");
        Ok(())
    }

    /// Add authentication to request parameters
    pub async fn add_auth_params(&self, params: &mut HashMap<String, String>) -> Result<()> {
        if let Some(session) = self.get_session().await? {
            params.insert("sk".to_string(), session.key);
            // Don't sign locally - the worker will handle signing for authenticated requests
        }

        Ok(())
    }
}

/// Authentication status
#[derive(Debug, Serialize)]
pub struct AuthStatus {
    pub authenticated: bool,
    pub username: Option<String>,
}

impl AuthStatus {
    pub fn new(session: Option<Session>) -> Self {
        match session {
            Some(s) => Self {
                authenticated: true,
                username: Some(s.username),
            },
            None => Self {
                authenticated: false,
                username: None,
            },
        }
    }
}
