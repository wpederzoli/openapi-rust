use std::{fs, path::Path};

pub struct Parser;

impl Parser {
    pub fn validate_extension(file: &str) -> bool {
        file.ends_with(".yaml") || file.ends_with(".yml")
    }

    pub fn parse_openapi(file_path: &Path) -> Result<String, String> {
        match fs::read_to_string(file_path) {
            Ok(content) => Ok(content),
            Err(_e) => Err("Failed to read OpenAPI file".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_extension() {
        let test_yaml = "test.yaml";
        let test_yml = "test.yml";
        assert!(Parser::validate_extension(test_yaml));
        assert!(Parser::validate_extension(test_yml));
    }

    #[test]
    fn test_invalid_extension() {
        let test_json = "test.json";
        assert!(!Parser::validate_extension(test_json));
    }

    #[test]
    fn test_parse_file() {
        let file = Path::new("tests/openapi_test.yaml");
        let result = Parser::parse_openapi(file);

        assert!(
            result.is_ok(),
            "Parsing should succeed for a valid OpenAPI file"
        );
    }

    #[test]
    fn test_parse_file_failed() {
        let file = Path::new("tests/openapi_test_invalid.yaml");
        let result = Parser::parse_openapi(file);

        assert!(result.is_err(), "Invalid file should fail with an error");
    }
}
