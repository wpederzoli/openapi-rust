use std::env;
mod output_messages;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args[1] == "-h" || args[1] == "--help" {
        println!("{}", output_messages::HELP_MESSAGE);
    } else {
        println!("{}", output_messages::ERROR_MESSAGE);
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

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(
            stdout.contains(&output_messages::ERROR_MESSAGE),
            "Error message should be displayed"
        );
    }
}
