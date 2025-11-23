use crate::enums::CommandType;
use crate::enums::ProviderType;
use crate::errors::config_error::ConfigError;
use std::str::FromStr;

pub fn parse_args(args: &[String]) -> Result<CommandType, ConfigError> {
    let index = 1;
    while index < args.len() {
        let arg = &args[index];
        match arg.as_str() {
            "configure" => {
                if index + 1 >= args.len() {
                    return Err(ConfigError::InvalidArgument(format!(
                        "Provider not provided"
                    )));
                }
                let provider = ProviderType::from_str(args[index + 1].as_str())?;
                let command = CommandType::Configure(provider);
                return Ok(command);
            }
            _ => {
                return Err(ConfigError::InvalidConfig(format!(
                    "Unknown argument: {}",
                    arg
                )))
            }
        }
    }
    Ok(CommandType::Get)
}
