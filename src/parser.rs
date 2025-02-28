use std::{fs, path::Path};

use crate::output_messages;

pub struct Parser {
    openapi: String,
}

impl Parser {
    pub fn new(file_path: &Path) -> Result<Self, String> {
        if !Parser::validate_extension(file_path.to_str().unwrap()) {
            eprintln!("{}", output_messages::UNSUPPORTED_FILETYPE_MESSAGE);
            return Err(output_messages::UNSUPPORTED_FILETYPE_MESSAGE.to_string());
        }

        let openapi = Parser::parse_openapi(file_path);

        match openapi {
            Ok(openapi) => Ok(Parser { openapi }),
            Err(_) => Err(output_messages::ERROR_MESSAGE.to_string()),
        }
    }

    fn validate_extension(file: &str) -> bool {
        file.ends_with(".yaml") || file.ends_with(".yml")
    }

    fn parse_openapi(file_path: &Path) -> Result<String, String> {
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
    fn test_create_invalid_extension() {
        let test_json = "./test.json";
        let result = Parser::new(Path::new(test_json));

        assert!(result.is_err(), "Unsupported format should throw an error");
    }

    #[test]
    fn test_create_invalid_file() {
        let test_file = "tests/openapi_test_invalid.yml";
        let result = Parser::new(Path::new(test_file));

        assert!(result.is_err(), "Invalid file should fail with an error");
    }

    #[test]
    fn test_create_valid_file() {
        let test_file = "src/tests/openapi_test.yaml";
        let result = Parser::new(Path::new(test_file));

        assert!(result.is_ok(), "Valid file should not throw an error");
    }
}
