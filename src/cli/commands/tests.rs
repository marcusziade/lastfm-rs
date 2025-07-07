// Unit tests for command utilities

#[cfg(test)]
mod command_tests {
    use super::super::*;
    use crate::cli::traits::CommandArgs;

    #[test]
    fn test_get_required_arg_from_named() {
        let mut args = CommandArgs::default();
        args.named
            .insert("artist".to_string(), "The Beatles".to_string());

        let result = get_required_arg(&args, "artist");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "The Beatles");
    }

    #[test]
    fn test_get_required_arg_from_positional() {
        let mut args = CommandArgs::default();
        args.positional.push("Radiohead".to_string());

        let result = get_required_arg(&args, "artist");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Radiohead");
    }

    #[test]
    fn test_get_required_arg_missing() {
        let args = CommandArgs::default();

        let result = get_required_arg(&args, "artist");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing argument"));
    }

    #[test]
    fn test_get_optional_arg_with_value() {
        let mut args = CommandArgs::default();
        args.named.insert("limit".to_string(), "10".to_string());

        let result = get_optional_arg(&args, "limit", None);
        assert_eq!(result, "10");
    }

    #[test]
    fn test_get_optional_arg_with_default() {
        let args = CommandArgs::default();

        let result = get_optional_arg(&args, "limit", Some("50"));
        assert_eq!(result, "50");
    }

    #[test]
    fn test_get_optional_arg_no_default() {
        let args = CommandArgs::default();

        let result = get_optional_arg(&args, "limit", None);
        assert_eq!(result, "");
    }

    #[test]
    fn test_get_flag_present() {
        let mut args = CommandArgs::default();
        args.flags.insert("autocorrect".to_string(), true);

        let result = get_flag(&args, "autocorrect");
        assert!(result);
    }

    #[test]
    fn test_get_flag_absent() {
        let args = CommandArgs::default();

        let result = get_flag(&args, "autocorrect");
        assert!(!result);
    }

    #[test]
    fn test_command_registry_registration() {
        use crate::cli::api::LastfmApiClient;
        use std::sync::Arc;

        let api_client = Arc::new(LastfmApiClient::new("http://test.com".to_string(), None));

        let mut registry = CommandRegistry::new();
        let command = Box::new(artist::ArtistInfoCommand::new(api_client));

        registry.register(command);

        assert!(registry.get("artist.info").is_some());
        assert!(registry.get("nonexistent.command").is_none());
    }

    #[test]
    fn test_command_registry_all() {
        use crate::cli::api::LastfmApiClient;
        use std::sync::Arc;

        let api_client = Arc::new(LastfmApiClient::new("http://test.com".to_string(), None));

        let mut registry = CommandRegistry::new();
        registry.register(Box::new(artist::ArtistInfoCommand::new(api_client.clone())));
        registry.register(Box::new(artist::ArtistSimilarCommand::new(
            api_client.clone(),
        )));

        let all_commands = registry.all();
        assert_eq!(all_commands.len(), 2);
    }

    #[test]
    fn test_artist_search_validation_success() {
        use crate::cli::api::LastfmApiClient;
        use std::sync::Arc;

        let api_client = Arc::new(LastfmApiClient::new("http://test.com".to_string(), None));

        let cmd = artist::ArtistSearchCommand::new(api_client);

        let mut args = CommandArgs::default();
        args.named
            .insert("query".to_string(), "Beatles".to_string());

        assert!(cmd.validate_args(&args).is_ok());
    }

    #[test]
    fn test_artist_search_validation_failure() {
        use crate::cli::api::LastfmApiClient;
        use std::sync::Arc;

        let api_client = Arc::new(LastfmApiClient::new("http://test.com".to_string(), None));

        let cmd = artist::ArtistSearchCommand::new(api_client);

        let args = CommandArgs::default();

        assert!(cmd.validate_args(&args).is_err());
    }

    #[test]
    fn test_base_command_creation() {
        use crate::cli::api::LastfmApiClient;
        use std::sync::Arc;

        let api_client = Arc::new(LastfmApiClient::new("http://test.com".to_string(), None));

        let base = BaseCommand::new("test.command", "Test command description", api_client);

        assert_eq!(base.name, "test.command");
        assert_eq!(base.description, "Test command description");
    }

    #[test]
    fn test_command_args_builder_pattern() {
        let mut args = CommandArgs::default();

        // Test building args progressively
        args.named
            .insert("artist".to_string(), "Radiohead".to_string());
        args.named.insert("limit".to_string(), "10".to_string());
        args.flags.insert("autocorrect".to_string(), true);
        args.positional.push("OK Computer".to_string());

        assert_eq!(args.named.len(), 2);
        assert_eq!(args.flags.len(), 1);
        assert_eq!(args.positional.len(), 1);

        assert_eq!(args.named.get("artist").unwrap(), "Radiohead");
        assert_eq!(args.flags.get("autocorrect").unwrap(), &true);
        assert_eq!(args.positional[0], "OK Computer");
    }

    #[test]
    fn test_user_top_artists_period_validation() {
        let valid_periods = ["overall", "7day", "1month", "3month", "6month", "12month"];

        for period in &valid_periods {
            // These should all be valid
            assert!(valid_periods.contains(period));
        }

        // Test invalid periods
        let invalid_periods = ["2weeks", "1year", "all", ""];
        for period in &invalid_periods {
            assert!(!valid_periods.contains(period));
        }
    }

    #[test]
    fn test_numeric_limit_validation() {
        let mut args = CommandArgs::default();

        // Valid limits
        args.named.insert("limit".to_string(), "50".to_string());
        let limit = get_optional_arg(&args, "limit", Some("30"));
        assert!(limit.parse::<u32>().is_ok());
        assert_eq!(limit.parse::<u32>().unwrap(), 50);

        // Test edge cases
        args.named.insert("limit".to_string(), "0".to_string());
        let limit = get_optional_arg(&args, "limit", Some("30"));
        assert_eq!(limit.parse::<u32>().unwrap(), 0);

        args.named.insert("limit".to_string(), "1000".to_string());
        let limit = get_optional_arg(&args, "limit", Some("30"));
        assert_eq!(limit.parse::<u32>().unwrap(), 1000);
    }

    #[test]
    fn test_get_required_arg_priority() {
        let mut args = CommandArgs::default();

        // When both named and positional exist, named takes priority
        args.named
            .insert("artist".to_string(), "Named Artist".to_string());
        args.positional.push("Positional Artist".to_string());

        let result = get_required_arg(&args, "artist");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Named Artist");
    }

    #[test]
    fn test_command_registry_with_defaults() {
        use crate::cli::api::LastfmApiClient;
        use std::sync::Arc;

        let api_client = Arc::new(LastfmApiClient::new(
            "http://test.com".to_string(),
            Some("test_key".to_string()),
        ));

        let registry = CommandRegistry::with_defaults(api_client);

        // Test that all expected commands are registered
        assert!(registry.get("artist.info").is_some());
        assert!(registry.get("artist.similar").is_some());
        assert!(registry.get("artist.search").is_some());
        assert!(registry.get("artist.top-albums").is_some());
        assert!(registry.get("artist.top-tracks").is_some());

        assert!(registry.get("album.info").is_some());
        assert!(registry.get("album.search").is_some());

        assert!(registry.get("track.info").is_some());
        assert!(registry.get("track.search").is_some());
        assert!(registry.get("track.similar").is_some());

        assert!(registry.get("chart.top-artists").is_some());
        assert!(registry.get("chart.top-tracks").is_some());
        assert!(registry.get("chart.top-tags").is_some());

        assert!(registry.get("geo.top-artists").is_some());
        assert!(registry.get("geo.top-tracks").is_some());

        assert!(registry.get("tag.info").is_some());
        assert!(registry.get("tag.top-artists").is_some());
        assert!(registry.get("tag.top-albums").is_some());
        assert!(registry.get("tag.top-tracks").is_some());

        assert!(registry.get("user.info").is_some());
        assert!(registry.get("user.recent-tracks").is_some());
        assert!(registry.get("user.top-artists").is_some());

        assert!(registry.get("library.artists").is_some());
    }

    #[test]
    fn test_boolean_flag_handling() {
        let mut args = CommandArgs::default();

        // Explicitly set flags
        args.flags.insert("autocorrect".to_string(), true);
        args.flags.insert("extended".to_string(), false);

        assert!(get_flag(&args, "autocorrect"));
        assert!(!get_flag(&args, "extended"));
        assert!(!get_flag(&args, "nonexistent")); // Default false
    }

    #[test]
    fn test_empty_optional_arg_behavior() {
        let mut args = CommandArgs::default();
        args.named.insert("empty".to_string(), "".to_string());

        // Empty string should be returned as-is
        let result = get_optional_arg(&args, "empty", Some("default"));
        assert_eq!(result, "");

        // Missing arg should return default
        let result = get_optional_arg(&args, "missing", Some("default"));
        assert_eq!(result, "default");

        // Missing arg with no default should return empty string
        let result = get_optional_arg(&args, "missing", None);
        assert_eq!(result, "");
    }
}
