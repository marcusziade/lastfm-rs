// Additional unit tests for configuration management

#[cfg(test)]
mod additional_tests {
    use super::super::*;
    use std::fs;
    use tempfile::TempDir;
    
    fn setup_test_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }
    
    #[test]
    fn test_cli_config_default() {
        let config = CliConfig::default();
        
        assert_eq!(config.worker_url, "https://lastfm-proxy-worker.guitaripod.workers.dev");
        assert_eq!(config.output_format, OutputFormat::Pretty);
        assert!(config.color_output);
        assert_eq!(config.request_timeout_secs, 30);
        assert_eq!(config.cache_ttl, 3600);
        assert_eq!(config.interactive_history_size, 1000);
        assert_eq!(config.api_key, Some("REDACTED_API_KEY".to_string()));
    }
    
    #[test]
    fn test_config_field_descriptions() {
        assert_eq!(
            ConfigField::WorkerUrl.description(),
            "URL of the Last.fm proxy worker"
        );
        assert_eq!(
            ConfigField::ApiKey.description(),
            "Last.fm API key (optional if using proxy)"
        );
        assert_eq!(
            ConfigField::OutputFormat.description(),
            "Default output format (json, table, pretty, compact)"
        );
    }
    
    #[test]
    fn test_config_field_all_variants() {
        let fields = vec![
            ConfigField::WorkerUrl,
            ConfigField::ApiKey,
            ConfigField::OutputFormat,
            ConfigField::CacheTtl,
            ConfigField::InteractiveHistorySize,
            ConfigField::ColorOutput,
            ConfigField::RequestTimeoutSecs,
        ];
        
        for field in fields {
            let str_repr = field.as_str();
            let reconstructed = ConfigField::from_str(str_repr);
            assert!(reconstructed.is_some(), "Failed to reconstruct {:?}", field);
        }
    }
    
    #[test]
    fn test_config_get_api_key_none() {
        let mut config = CliConfig::default();
        config.api_key = None;
        
        assert_eq!(config.get_value(ConfigField::ApiKey), "Not set");
    }
    
    #[test]
    fn test_config_set_api_key_empty() {
        let mut config = CliConfig::default();
        
        config.set_value(ConfigField::ApiKey, "").unwrap();
        assert_eq!(config.api_key, None);
        
        config.set_value(ConfigField::ApiKey, "Not set").unwrap();
        assert_eq!(config.api_key, None);
    }
    
    #[test]
    fn test_config_set_invalid_values() {
        let mut config = CliConfig::default();
        
        // Invalid cache TTL
        let result = config.set_value(ConfigField::CacheTtl, "not_a_number");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid cache TTL"));
        
        // Invalid boolean
        let result = config.set_value(ConfigField::ColorOutput, "not_a_bool");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid boolean value"));
        
        // Invalid output format
        let result = config.set_value(ConfigField::OutputFormat, "xml");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid output format"));
    }
    
    #[test]
    fn test_config_manager_validation() {
        let manager = ConfigManager::new().unwrap();
        
        // Valid config
        let valid_config = CliConfig::default();
        assert!(manager.validate(&valid_config).is_ok());
        
        // Invalid: empty worker URL
        let mut invalid_config = CliConfig::default();
        invalid_config.worker_url = String::new();
        let result = manager.validate(&invalid_config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Worker URL cannot be empty"));
        
        // Invalid: zero timeout
        let mut invalid_config = CliConfig::default();
        invalid_config.request_timeout_secs = 0;
        let result = manager.validate(&invalid_config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Request timeout must be greater than 0"));
        
        // Invalid: zero cache TTL
        let mut invalid_config = CliConfig::default();
        invalid_config.cache_ttl = 0;
        let result = manager.validate(&invalid_config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cache TTL must be greater than 0"));
    }
    
    #[tokio::test]
    async fn test_config_save_and_load() {
        let temp_dir = setup_test_dir();
        let config_path = temp_dir.path().join("config.toml");
        
        let config = CliConfig {
            worker_url: "https://test.worker.dev".to_string(),
            api_key: Some("secret-key".to_string()),
            output_format: OutputFormat::Json,
            cache_ttl: 7200,
            interactive_history_size: 500,
            color_output: false,
            request_timeout_secs: 45,
        };
        
        // Save config
        let toml_str = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_path, toml_str).unwrap();
        
        // Load config
        let loaded_str = fs::read_to_string(&config_path).unwrap();
        let loaded_config: CliConfig = toml::from_str(&loaded_str).unwrap();
        
        assert_eq!(loaded_config.worker_url, config.worker_url);
        assert_eq!(loaded_config.api_key, config.api_key);
        assert_eq!(loaded_config.output_format, config.output_format);
        assert_eq!(loaded_config.color_output, config.color_output);
        assert_eq!(loaded_config.request_timeout_secs, config.request_timeout_secs);
        assert_eq!(loaded_config.cache_ttl, config.cache_ttl);
        assert_eq!(loaded_config.interactive_history_size, config.interactive_history_size);
    }
    
    #[test]
    fn test_partial_config_deserialization() {
        // Test that partial configs work (missing fields use defaults)
        let partial_toml = r#"
            worker_url = "https://partial.worker.dev"
            output_format = "json"
        "#;
        
        let config: CliConfig = toml::from_str(partial_toml).unwrap();
        
        assert_eq!(config.worker_url, "https://partial.worker.dev");
        assert_eq!(config.output_format, OutputFormat::Json);
        // These should be defaults
        assert!(config.color_output);
        assert_eq!(config.request_timeout_secs, 30);
        assert_eq!(config.cache_ttl, 3600);
        assert_eq!(config.api_key, Some("REDACTED_API_KEY".to_string()));
    }
    
    #[test]
    fn test_config_field_from_str() {
        assert_eq!(ConfigField::from_str("worker_url"), Some(ConfigField::WorkerUrl));
        assert_eq!(ConfigField::from_str("api_key"), Some(ConfigField::ApiKey));
        assert_eq!(ConfigField::from_str("output_format"), Some(ConfigField::OutputFormat));
        assert_eq!(ConfigField::from_str("cache_ttl"), Some(ConfigField::CacheTtl));
        assert_eq!(ConfigField::from_str("interactive_history_size"), Some(ConfigField::InteractiveHistorySize));
        assert_eq!(ConfigField::from_str("color_output"), Some(ConfigField::ColorOutput));
        assert_eq!(ConfigField::from_str("request_timeout_secs"), Some(ConfigField::RequestTimeoutSecs));
        assert_eq!(ConfigField::from_str("invalid_field"), None);
    }
    
    #[test]
    fn test_config_get_value() {
        let config = CliConfig {
            worker_url: "https://test.dev".to_string(),
            api_key: Some("test_key".to_string()),
            output_format: OutputFormat::Table,
            cache_ttl: 7200,
            interactive_history_size: 2000,
            color_output: false,
            request_timeout_secs: 60,
        };
        
        assert_eq!(config.get_value(ConfigField::WorkerUrl), "https://test.dev");
        assert_eq!(config.get_value(ConfigField::ApiKey), "test_key");
        assert_eq!(config.get_value(ConfigField::OutputFormat), "table");
        assert_eq!(config.get_value(ConfigField::CacheTtl), "7200");
        assert_eq!(config.get_value(ConfigField::InteractiveHistorySize), "2000");
        assert_eq!(config.get_value(ConfigField::ColorOutput), "false");
        assert_eq!(config.get_value(ConfigField::RequestTimeoutSecs), "60");
    }
    
    #[test]
    fn test_config_set_value_all_fields() {
        let mut config = CliConfig::default();
        
        // Test setting each field
        config.set_value(ConfigField::WorkerUrl, "https://new.dev").unwrap();
        assert_eq!(config.worker_url, "https://new.dev");
        
        config.set_value(ConfigField::ApiKey, "new_key").unwrap();
        assert_eq!(config.api_key, Some("new_key".to_string()));
        
        config.set_value(ConfigField::OutputFormat, "compact").unwrap();
        assert_eq!(config.output_format, OutputFormat::Compact);
        
        config.set_value(ConfigField::CacheTtl, "1800").unwrap();
        assert_eq!(config.cache_ttl, 1800);
        
        config.set_value(ConfigField::InteractiveHistorySize, "5000").unwrap();
        assert_eq!(config.interactive_history_size, 5000);
        
        config.set_value(ConfigField::ColorOutput, "true").unwrap();
        assert_eq!(config.color_output, true);
        
        config.set_value(ConfigField::RequestTimeoutSecs, "90").unwrap();
        assert_eq!(config.request_timeout_secs, 90);
    }
    
    #[test]
    fn test_output_format_conversions() {
        // Test all output format string conversions
        let formats = vec![
            (OutputFormat::Json, "json"),
            (OutputFormat::Table, "table"),
            (OutputFormat::Pretty, "pretty"),
            (OutputFormat::Compact, "compact"),
        ];
        
        for (format, expected_str) in formats {
            // Test serialization
            let serialized = serde_json::to_string(&format).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));
            
            // Test deserialization
            let deserialized: OutputFormat = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, format);
        }
    }
    
    #[test]
    fn test_config_edge_cases() {
        let mut config = CliConfig::default();
        
        // Test setting boolean values with various formats
        config.set_value(ConfigField::ColorOutput, "yes").unwrap();
        assert!(config.color_output);
        
        config.set_value(ConfigField::ColorOutput, "no").unwrap();
        assert!(!config.color_output);
        
        config.set_value(ConfigField::ColorOutput, "1").unwrap();
        assert!(config.color_output);
        
        config.set_value(ConfigField::ColorOutput, "0").unwrap();
        assert!(!config.color_output);
        
        // Test large timeout values
        config.set_value(ConfigField::RequestTimeoutSecs, "300").unwrap();
        assert_eq!(config.request_timeout_secs, 300);
        
        // Test large cache TTL
        config.set_value(ConfigField::CacheTtl, "86400").unwrap(); // 24 hours
        assert_eq!(config.cache_ttl, 86400);
    }
    
    #[test] 
    fn test_config_manager_error_handling() {
        let temp_dir = setup_test_dir();
        let invalid_path = temp_dir.path().join("nonexistent/dir/config.toml");
        
        // Should handle missing parent directory gracefully
        let manager_result = ConfigManager::with_path(invalid_path);
        assert!(manager_result.is_ok()); // Manager creation should succeed
        
        let manager = manager_result.unwrap();
        
        // Save should create parent directories
        let config = CliConfig::default();
        let save_result = manager.save(&config);
        assert!(save_result.is_ok());
    }
    
    #[test]
    fn test_default_config_matches_production() {
        let config = CliConfig::default();
        
        // Verify the default configuration points to production
        assert_eq!(config.worker_url, "https://lastfm-proxy-worker.guitaripod.workers.dev");
        
        // Verify default API key is set
        assert!(config.api_key.is_some());
        assert_eq!(config.api_key.unwrap(), "REDACTED_API_KEY");
    }
}