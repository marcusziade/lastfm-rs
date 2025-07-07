// Unit tests for output formatting

#[cfg(test)]
mod output_tests {
    use super::super::*;
    use crate::cli::traits::{CommandOutput, OutputMetadata};
    use serde_json::json;

    #[test]
    fn test_format_search_artists_table() {
        let data = json!({
            "artist": [
                {
                    "name": "The Beatles",
                    "listeners": "5806609",
                    "url": "https://www.last.fm/music/The+Beatles"
                },
                {
                    "name": "Beatles Revival Band",
                    "listeners": "45387",
                    "url": "https://www.last.fm/music/Beatles+Revival+Band"
                }
            ]
        });

        let result = format_search_artists_table(&data);
        assert!(result.is_some());
        let table = result.unwrap();

        // Check that table contains headers
        assert!(table.contains("Artist"));
        assert!(table.contains("Listeners"));
        assert!(table.contains("URL"));

        // Check that data is included
        assert!(table.contains("The Beatles"));
        assert!(table.contains("5806609"));
        assert!(table.contains("Beatles Revival Band"));
    }

    #[test]
    fn test_format_empty_search_results() {
        let data = json!({
            "artist": []
        });

        let result = format_search_artists_table(&data);
        assert!(result.is_some());
        let table = result.unwrap();

        // Should still have headers
        assert!(table.contains("Artist"));
        assert!(table.contains("Listeners"));

        // But no data rows (only header and separators)
        assert_eq!(table.matches('\n').count(), 3); // Header + separator + empty table
    }

    #[test]
    fn test_format_track_with_object_artist() {
        let data = json!({
            "track": [
                {
                    "name": "Let It Be",
                    "artist": {
                        "name": "The Beatles",
                        "mbid": "b10bbbfc-cf9e-42e0-be17-e2c3e1d2600d"
                    },
                    "listeners": "1024181"
                }
            ]
        });

        let result = format_tracks_list_table(&data);
        assert!(result.is_some());
        let table = result.unwrap();

        // Check that artist name is extracted from object
        assert!(table.contains("The Beatles"));
        assert!(table.contains("Let It Be"));
    }

    #[test]
    fn test_format_track_with_string_artist() {
        let data = json!({
            "track": [
                {
                    "name": "Imagine",
                    "artist": "John Lennon",
                    "listeners": "2541234"
                }
            ]
        });

        let result = format_tracks_list_table(&data);
        assert!(result.is_some());
        let table = result.unwrap();

        // Check that string artist is handled correctly
        assert!(table.contains("John Lennon"));
        assert!(table.contains("Imagine"));
    }

    #[test]
    fn test_output_format_enum_serialization() {
        let json_format = OutputFormat::Json;
        let serialized = serde_json::to_string(&json_format).unwrap();
        assert_eq!(serialized, "\"json\"");

        let table_format = OutputFormat::Table;
        let serialized = serde_json::to_string(&table_format).unwrap();
        assert_eq!(serialized, "\"table\"");
    }

    #[test]
    fn test_json_formatter_pretty() {
        let formatter = JsonFormatter::new(true);
        let output = CommandOutput {
            data: json!({
                "test": "value",
                "nested": {
                    "key": "value"
                }
            }),
            metadata: OutputMetadata {
                cache_hit: false,
                response_time_ms: 100,
                api_calls_made: 1,
            },
        };

        let formatted = formatter.format(&output);
        assert!(formatted.contains("\"test\": \"value\""));
        assert!(formatted.contains("\n")); // Pretty formatting includes newlines
    }

    #[test]
    fn test_json_formatter_compact() {
        let formatter = JsonFormatter::new(false);
        let output = CommandOutput {
            data: json!({"test": "value"}),
            metadata: OutputMetadata {
                cache_hit: false,
                response_time_ms: 100,
                api_calls_made: 1,
            },
        };

        let formatted = formatter.format(&output);
        assert_eq!(formatted, "{\"test\":\"value\"}");
    }

    #[test]
    fn test_format_value_as_table_with_search_results() {
        let search_results = json!({
            "results": {
                "artistmatches": {
                    "artist": [
                        {
                            "name": "Test Artist",
                            "listeners": "12345",
                            "url": "https://test.url"
                        }
                    ]
                }
            }
        });

        let result = format_value_as_table(&search_results);
        assert!(result.is_some());
        assert!(result.unwrap().contains("Test Artist"));
    }

    #[test]
    fn test_format_generic_object_table() {
        let mut map = serde_json::Map::new();
        map.insert("key1".to_string(), json!("value1"));
        map.insert("key2".to_string(), json!(42));
        map.insert("key3".to_string(), json!(true));
        map.insert("key4".to_string(), json!(null));

        let result = format_generic_object_table(&map);
        assert!(result.is_some());
        let table = result.unwrap();

        assert!(table.contains("key1"));
        assert!(table.contains("value1"));
        assert!(table.contains("42"));
        assert!(table.contains("true"));
        assert!(table.contains("null"));
    }

    #[test]
    fn test_format_error_pretty_with_color() {
        let error = json!(6);
        let message = json!("Invalid parameters");

        let result = format_error_pretty(&error, Some(&message), false);
        assert!(result.contains("Error 6"));
        assert!(result.contains("Invalid parameters"));
    }

    #[test]
    fn test_create_formatter_returns_correct_type() {
        let json_formatter = create_formatter(OutputFormat::Json, false);
        assert_eq!(json_formatter.name(), "json-pretty");

        let table_formatter = create_formatter(OutputFormat::Table, false);
        assert_eq!(table_formatter.name(), "table");

        let pretty_formatter = create_formatter(OutputFormat::Pretty, true);
        assert_eq!(pretty_formatter.name(), "pretty");

        let compact_formatter = create_formatter(OutputFormat::Compact, false);
        assert_eq!(compact_formatter.name(), "json-compact");
    }

    #[test]
    fn test_format_tags_table() {
        let tags = json!({
            "tag": [
                {
                    "name": "rock",
                    "taggings": "4056273",
                    "reach": "402102"
                },
                {
                    "name": "electronic",
                    "taggings": "2483420",
                    "reach": "261280"
                }
            ]
        });

        let result = format_tags_table(&tags);
        assert!(result.is_some());
        let table = result.unwrap();

        assert!(table.contains("rock"));
        assert!(table.contains("4056273"));
        assert!(table.contains("402102"));
        assert!(table.contains("electronic"));
        assert!(table.contains("2483420"));
        assert!(table.contains("261280"));
    }

    #[test]
    fn test_format_similar_tracks_table() {
        let similar = json!({
            "track": [
                {
                    "name": "Karma Police",
                    "artist": {
                        "name": "Radiohead"
                    },
                    "match": 0.893
                },
                {
                    "name": "No Surprises",
                    "artist": {
                        "name": "Radiohead"
                    },
                    "match": 1.0
                }
            ]
        });

        let result = format_similar_tracks_table(&similar);
        assert!(result.is_some());
        let table = result.unwrap();

        assert!(table.contains("Karma Police"));
        assert!(table.contains("Radiohead"));
        assert!(table.contains("89.3"));
        assert!(table.contains("No Surprises"));
        assert!(table.contains("100.0"));
    }

    #[test]
    fn test_format_recent_tracks_table() {
        let recent = json!({
            "track": [
                {
                    "name": "We Will Rock You",
                    "artist": {
                        "name": "Queen"
                    },
                    "album": {
                        "#text": "News of the World"
                    },
                    "date": {
                        "#text": "06 Jul 2025, 14:21",
                        "uts": "1751811712"
                    }
                },
                {
                    "name": "Currently Playing",
                    "artist": {
                        "name": "Artist"
                    },
                    "album": {
                        "#text": "Album"
                    },
                    "@attr": {
                        "nowplaying": "true"
                    }
                }
            ]
        });

        let result = format_recent_tracks_table(&recent);
        assert!(result.is_some());
        let table = result.unwrap();

        assert!(table.contains("We Will Rock You"));
        assert!(table.contains("Queen"));
        assert!(table.contains("News of the World"));
        assert!(table.contains("06 Jul 2025, 14:21"));
        assert!(table.contains("Currently Playing"));
        assert!(table.contains("Now Playing"));
    }

    #[test]
    fn test_format_similar_artists_table() {
        let similar = json!({
            "artist": [
                {
                    "name": "Thom Yorke",
                    "match": "1.0"
                },
                {
                    "name": "Atoms for Peace",
                    "match": "0.9384"
                }
            ]
        });

        let result = format_similar_artists_table(&similar);
        assert!(result.is_some());
        let table = result.unwrap();

        assert!(table.contains("Thom Yorke"));
        assert!(table.contains("100.0"));
        assert!(table.contains("Atoms for Peace"));
        assert!(table.contains("93.8"));
    }

    #[test]
    fn test_table_formatter_with_metadata() {
        let formatter = TableFormatter;
        let output = CommandOutput {
            data: json!({
                "artist": {
                    "name": "Test Artist",
                    "stats": {
                        "listeners": "1000",
                        "playcount": "5000"
                    },
                    "url": "https://test.url"
                }
            }),
            metadata: OutputMetadata {
                cache_hit: true,
                response_time_ms: 50,
                api_calls_made: 1,
            },
        };

        let formatted = formatter.format(&output);
        assert!(formatted.contains("Response time: 50ms"));
        assert!(formatted.contains("Cache: HIT"));
        assert!(formatted.contains("API calls: 1"));
        assert!(formatted.contains("Test Artist"));
    }

    #[test]
    fn test_format_array_table_empty() {
        let array: Vec<Value> = vec![];
        let result = format_array_table(&array);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "Empty array");
    }

    #[test]
    fn test_format_array_table_with_values() {
        let array = vec![json!("string value"), json!(42), json!(true), json!(null)];

        let result = format_array_table(&array);
        assert!(result.is_some());
        let table = result.unwrap();

        assert!(table.contains("string value"));
        assert!(table.contains("42"));
        assert!(table.contains("true"));
        assert!(table.contains("null"));
    }
}
