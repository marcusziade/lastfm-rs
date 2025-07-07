// Unit tests for error handling

#[cfg(test)]
mod error_tests {
    use super::super::*;

    #[test]
    fn test_cli_error_messages() {
        let config_error = CliError::config("Invalid configuration");
        assert_eq!(
            config_error.to_string(),
            "Configuration error: Invalid configuration"
        );

        let api_error = CliError::api("Invalid parameters");
        assert_eq!(api_error.to_string(), "API error: Invalid parameters");

        let invalid_command = CliError::invalid_command("unknown.command");
        assert_eq!(
            invalid_command.to_string(),
            "Invalid command: unknown.command"
        );

        let missing_arg = CliError::missing_argument("artist");
        assert_eq!(missing_arg.to_string(), "Missing argument: artist");

        let validation_error = CliError::validation("Artist name required");
        assert_eq!(
            validation_error.to_string(),
            "Validation error: Artist name required"
        );

        let cache_error = CliError::cache("Cache expired");
        assert_eq!(cache_error.to_string(), "Cache error: Cache expired");

        let worker_error = CliError::worker("Worker unavailable");
        assert_eq!(worker_error.to_string(), "Worker error: Worker unavailable");

        let not_found = CliError::not_found("Resource not found");
        assert_eq!(not_found.to_string(), "Not found: Resource not found");

        let rate_limit = CliError::RateLimit;
        assert_eq!(rate_limit.to_string(), "Rate limit exceeded");

        let auth_required = CliError::AuthRequired;
        assert_eq!(auth_required.to_string(), "Authentication required");

        let other_error = CliError::other("Unexpected error");
        assert_eq!(other_error.to_string(), "Unexpected error");
    }

    #[test]
    fn test_is_retryable() {
        // Retryable errors
        assert!(CliError::RateLimit.is_retryable());
        assert!(CliError::api("Server error").is_retryable());
        assert!(CliError::cache("Cache miss").is_retryable());

        // Non-retryable errors
        assert!(!CliError::validation("Invalid input").is_retryable());
        assert!(!CliError::missing_argument("test").is_retryable());
        assert!(!CliError::invalid_command("bad").is_retryable());
        assert!(!CliError::AuthRequired.is_retryable());
        assert!(!CliError::not_found("test").is_retryable());
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(CliError::RateLimit.error_code(), 429);
        assert_eq!(CliError::AuthRequired.error_code(), 401);
        assert_eq!(CliError::not_found("test").error_code(), 404);
        assert_eq!(CliError::validation("test").error_code(), 400);
        assert_eq!(CliError::invalid_argument("test").error_code(), 400);
        assert_eq!(CliError::missing_argument("test").error_code(), 400);
        assert_eq!(CliError::api("test").error_code(), 500);
        assert_eq!(CliError::other("test").error_code(), 500);
    }

    #[test]
    fn test_result_type_alias() {
        fn test_function() -> Result<String> {
            Ok("Success".to_string())
        }

        let result = test_function();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
    }

    #[test]
    fn test_error_chain() {
        fn inner_function() -> Result<()> {
            Err(CliError::missing_argument("test"))
        }

        fn outer_function() -> Result<String> {
            inner_function()?;
            Ok("Never reached".to_string())
        }

        let result = outer_function();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing argument"));
    }

    #[test]
    fn test_from_io_error() {
        use std::io;

        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let cli_error: CliError = io_error.into();

        assert!(cli_error.to_string().contains("IO error"));
    }

    #[test]
    fn test_from_json_error() {
        let json_str = "{ invalid json }";
        let parse_result: Result<serde_json::Value> =
            serde_json::from_str(json_str).map_err(|e| e.into());

        assert!(parse_result.is_err());
        assert!(parse_result.unwrap_err().to_string().contains("JSON error"));
    }

    #[test]
    fn test_error_context_preservation() {
        // Test that error messages preserve important context
        let api_error = CliError::api("6: Invalid parameters - artist not found");
        assert!(api_error.to_string().contains("Invalid parameters"));
        assert!(api_error.to_string().contains("artist not found"));

        let validation_error = CliError::validation(
            "Period must be one of: overall, 7day, 1month, 3month, 6month, 12month",
        );
        assert!(validation_error.to_string().contains("Period must be"));
        assert!(validation_error.to_string().contains("7day"));
    }

    #[test]
    fn test_invalid_argument_error() {
        let err = CliError::invalid_argument("limit must be a positive integer");
        assert_eq!(
            err.to_string(),
            "Invalid argument: limit must be a positive integer"
        );
        assert_eq!(err.error_code(), 400);
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_error_pattern_matching() {
        let errors = vec![
            CliError::RateLimit,
            CliError::AuthRequired,
            CliError::api("Server error"),
            CliError::validation("Invalid input"),
        ];

        for error in errors {
            match error {
                CliError::RateLimit => assert_eq!(error.error_code(), 429),
                CliError::AuthRequired => assert_eq!(error.error_code(), 401),
                CliError::Api(_) => assert_eq!(error.error_code(), 500),
                CliError::Validation(_) => assert_eq!(error.error_code(), 400),
                _ => panic!("Unexpected error type"),
            }
        }
    }

    #[test]
    fn test_error_debug_format() {
        let err = CliError::missing_argument("artist");
        let debug_str = format!("{err:?}");

        // Debug format should include the variant name
        assert!(debug_str.contains("MissingArgument"));
        assert!(debug_str.contains("artist"));
    }

    #[test]
    fn test_common_error_scenarios() {
        // Missing required field
        let err = CliError::missing_argument("user");
        assert!(err.to_string().contains("user"));

        // Invalid format
        let err = CliError::validation("Date must be in UNIX timestamp format");
        assert!(err.to_string().contains("UNIX timestamp"));

        // API key issues
        let err = CliError::api("Invalid API key");
        assert!(err.to_string().contains("API key"));

        // Network issues
        let err = CliError::worker("Connection timeout");
        assert!(err.to_string().contains("Connection timeout"));
    }

    #[test]
    fn test_error_conversion_chain() {
        // Simulate error propagation through layers
        fn parse_limit(s: &str) -> Result<u32> {
            s.parse::<u32>()
                .map_err(|_| CliError::invalid_argument("limit must be a number"))
        }

        fn validate_limit(s: &str) -> Result<u32> {
            let limit = parse_limit(s)?;
            if limit > 1000 {
                Err(CliError::validation("limit cannot exceed 1000"))
            } else if limit == 0 {
                Err(CliError::validation("limit must be greater than 0"))
            } else {
                Ok(limit)
            }
        }

        assert!(validate_limit("50").is_ok());
        assert_eq!(validate_limit("50").unwrap(), 50);

        assert!(validate_limit("abc").is_err());
        assert!(validate_limit("abc")
            .unwrap_err()
            .to_string()
            .contains("must be a number"));

        assert!(validate_limit("2000").is_err());
        assert!(validate_limit("2000")
            .unwrap_err()
            .to_string()
            .contains("cannot exceed 1000"));

        assert!(validate_limit("0").is_err());
        assert!(validate_limit("0")
            .unwrap_err()
            .to_string()
            .contains("must be greater than 0"));
    }
}
