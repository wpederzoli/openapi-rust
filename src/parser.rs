use std::{fs, path::Path};

use serde::Deserialize;

use crate::output_messages;

#[derive(Debug, Deserialize)]
struct OpenApiSpec {
    openapi: String,
}

pub struct Parser {
    spec: OpenApiSpec,
}

impl Parser {
    pub fn new(file_path: &Path) -> Result<Self, String> {
        if !Parser::validate_extension(file_path.to_str().unwrap()) {
            eprintln!("{}", output_messages::UNSUPPORTED_FILETYPE_MESSAGE);
            return Err(output_messages::UNSUPPORTED_FILETYPE_MESSAGE.to_string());
        }

        let spec = Parser::parse_openapi(file_path)?;

        if !Parser::is_supported_version(&spec.openapi) {
            eprintln!("{}", output_messages::UNSUPPORTED_OPENAPI_VERSION);
            return Err(output_messages::UNSUPPORTED_OPENAPI_VERSION.to_string());
        }

        Ok(Parser { spec })
    }

    fn validate_extension(file: &str) -> bool {
        file.ends_with(".yaml") || file.ends_with(".yml")
    }

    fn parse_openapi(file_path: &Path) -> Result<OpenApiSpec, String> {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                let parsed_content = serde_yaml::from_str::<OpenApiSpec>(&content);
                match parsed_content {
                    Ok(content) => Ok(OpenApiSpec {
                        openapi: content.openapi,
                    }),
                    Err(_e) => Err("Failed to parse YAML file".to_string()),
                }
            }
            Err(_e) => Err("Failed to read OpenAPI file".to_string()),
        }
    }

    fn is_supported_version(version: &str) -> bool {
        version == "3.0.0" || version == "3.1.0"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_invalid_extension() {
        let test_json = "./test.json";
        let result = Parser::new(Path::new(test_json));

        assert!(result.is_err(), "Unsupported format should throw an error");
    }

    #[test]
    fn test_create_invalid_file() {
        let test_file = "tests/openapi_test_invalid.yaml";
        let result = Parser::new(Path::new(test_file));

        assert!(result.is_err(), "Invalid file should fail with an error");
    }

    #[test]
    fn test_create_valid_file() {
        let test_file = "src/tests/openapi_test.yaml";
        let result = Parser::new(Path::new(test_file));

        assert!(result.is_ok(), "Valid file should not throw an error");
    }

    #[test]
    fn test_openapi_valid_version() {
        let test_file = "src/tests/openapi_test.yaml";
        let parser = Parser::new(Path::new(test_file));

        assert!(parser.is_ok(), "Version 3.0.0 should be supported");
    }

    #[test]
    fn test_openapi_invalid_version() {
        let test_file = "src/tests/openapi_test_invalid.yaml";
        let parser = Parser::new(Path::new(test_file));

        assert!(
            parser.is_err(),
            "Only version 3.0.0 and 3.1.0 should be supported"
        );
    }
}
