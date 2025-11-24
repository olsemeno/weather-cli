use crate::enums::CommandType;
use crate::enums::ProviderType;
use crate::errors::config_error::ConfigError;
use std::str::FromStr;

pub fn parse_args(args: &[String]) -> Result<CommandType, ConfigError> {
    let index = 1;
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
        "get" => {
            return Ok(CommandType::Get(args[index + 1..].to_vec()));
        }
        "list" => {
            return Ok(CommandType::List);
        }
        "help" | "--help" | "-h" => {
            print_help();
            std::process::exit(0);
        }
        _ => {
            print_help();
            return Err(ConfigError::InvalidArgument(format!(
                "Unknown argument: {}",
                arg
            )))
        }
    }
}


pub fn print_help() {
    println!("Usage: weather <command> <params>\n");
    println!("Commands:\n");
    println!("  get <city> [days] - get weather for a city\n");
    println!("  list - list all cities\n");
    println!("  configure <provider> - configure the provider\n");
    println!("  help | --help | -h - print this help message\n");
}