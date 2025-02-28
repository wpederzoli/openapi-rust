use crate::output_messages;

pub enum CommandOption {
    Help,
    Generate(String),
}

impl CommandOption {
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        match args[1].as_str() {
            "-h" | "--help" => Ok(CommandOption::Help),
            "generate" => Ok(CommandOption::Generate(args[2].to_string())),
            _ => Err(output_messages::ERROR_MESSAGE.to_string()),
        }
    }
}
