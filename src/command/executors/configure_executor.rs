use crate::command::command_executor::CommandExecutor;
use crate::command::command_executor::ExecutionResult;
use crate::config::app_config::AppConfig;
use crate::enums::CommandType;
use crate::enums::ProviderType;
use crate::errors::execution_error::ExecutionError;

pub struct ConfigureExecutor;

pub struct ConfigureExecutionResult {
    pub provider: ProviderType,
}

impl ExecutionResult for ConfigureExecutionResult {
    fn get_printable_result(&self) -> String {
        format!("Provider configured successfully: {}", self.provider)
    }
}

impl CommandExecutor for ConfigureExecutor {
    fn execute(
        &self,
        command: CommandType,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        match command {
            CommandType::Configure(provider) => self.configure_provider(provider),
            _ => Err(Box::new(ExecutionError::InvalidCommand(
                format!("{}", command),
            ))),
        }
    }
}

impl ConfigureExecutionResult {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(provider: ProviderType) -> Box<dyn ExecutionResult> {
        Box::new(ConfigureExecutionResult { provider })
    }
}

impl ConfigureExecutor {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<dyn CommandExecutor> {
        Box::new(ConfigureExecutor)
    }
    fn configure_provider(
        &self,
        provider: ProviderType,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        match &mut AppConfig::get() {
            Some(app_config) => {
                app_config.set_provider(provider);
                match app_config.rewrite_config_file() {
                    Ok(_) => Ok(ConfigureExecutionResult::new(provider)),
                    Err(e) => Err(Box::new(e)),
                }
            }
            None => Err(Box::new(std::io::Error::other("App config not found"))),
        }
    }
}

