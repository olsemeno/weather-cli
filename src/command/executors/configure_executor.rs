use crate::command::command_executor::CommandExecutor;
use crate::command::command_executor::ExecutionResult;
use crate::config::app_config::AppConfig;
use crate::enums::CommandType;
use crate::enums::ProviderType;

pub struct ConfigureExecutor;

pub struct ConfigureExecutionResult {
    pub provider: ProviderType,
}

impl ExecutionResult for ConfigureExecutionResult {
    fn get_printable_result(&self) -> String {
        format!("Provider configured successfully: {}", self.provider.to_string())
    }
}

impl CommandExecutor for ConfigureExecutor {
    fn execute(
        &self,
        command: CommandType,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        match command {
            CommandType::Configure(provider) => self.configure_provider(provider),
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Invalid command",
                )))
            }
        }
    }
}

impl ConfigureExecutionResult {
    pub fn new(provider: ProviderType) -> Box<dyn ExecutionResult> {
        Box::new(ConfigureExecutionResult { provider })
    }
}

impl ConfigureExecutor {
    pub fn new() -> Box<dyn CommandExecutor> {
        Box::new(ConfigureExecutor)
    }
    fn configure_provider(
        &self,
        provider: ProviderType,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        match &mut AppConfig::get() {
            Some(app_config) => {
                app_config.provider = provider;
                match app_config.rewrite_config_file() {
                    Ok(_) => Ok(ConfigureExecutionResult::new(provider)),
                    Err(e) => return Err(Box::new(e)),
                }
            }
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "App config not found",
                )));
            }
        }
    }
}
