use std::{env, path::Path, process};

use commands::CommandOption;
mod commands;
mod output_messages;
mod parser;

use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("{}", output_messages::HELP_MESSAGE);
        process::exit(0);
    }

    match CommandOption::from_args(&args) {
        Ok(CommandOption::Help) => {
            println!("{}", output_messages::HELP_MESSAGE);
            process::exit(0);
        }
        Ok(CommandOption::Generate(file)) => {
            let _ = Parser::new(Path::new(&file));
        }
        Err(err) => {
            eprintln!("{}", err);
            eprintln!("{}", output_messages::ERROR_MESSAGE);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::output_messages;
    use std::process::Command;

    #[test]
    fn test_help_argument_present() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("-h")
            .output()
            .expect("Command execution failed!");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains(&output_messages::HELP_MESSAGE),
            "Output does not match expected"
        );

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--help")
            .output()
            .expect("Command execution failed!");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains(&output_messages::HELP_MESSAGE),
            "Output does not match expected"
        );
    }

    #[test]
    fn test_no_arguments_provided() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .output()
            .expect("Command execution failed");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains(&output_messages::HELP_MESSAGE),
            "Help message was not displayed"
        );
    }

    #[test]
    fn test_error_message() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("badparam")
            .output()
            .expect("Command execution failed");

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(
            stderr.contains(&output_messages::ERROR_MESSAGE),
            "Error message should be displayed"
        );
    }

    #[test]
    fn test_invalid_extension() {
        let file_path = "tests/openapi_test.json";

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("generate")
            .arg(file_path)
            .output()
            .expect("Command execution failed!");

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(
            stderr.contains(&output_messages::UNSUPPORTED_FILETYPE_MESSAGE),
            "Should have thrown unsupported file type"
        );
    }
}
