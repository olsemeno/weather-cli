use crate::command::command_executor::CommandExecutor;
use crate::command::command_executor::ExecutionResult;
use crate::enums::CommandType;
use crate::enums::ProviderType;
use crate::errors::execution_error::ExecutionError;
use crate::provider::provider_service::describe_provider;

pub struct ListExecutor;

pub struct ListExecutionResult {
    pub providers: Vec<(ProviderType, String)>,
}

impl ExecutionResult for ListExecutionResult {
    fn get_printable_result(&self) -> String {
        format!(
            "{}",
            self.providers
                .iter()
                .map(|p| format!("{}: {}", p.0.to_string(), p.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl CommandExecutor for ListExecutor {
    fn execute(
        &self,
        command: CommandType,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        match command {
            CommandType::List => self.list_providers(),
            _ => {
                return Err(Box::new(ExecutionError::InvalidCommand(
                    command.to_string(),
                )))
            }
        }
    }
}

impl ListExecutor {
    pub fn new() -> Box<dyn CommandExecutor> {
        Box::new(ListExecutor)
    }
    fn list_providers(
        &self,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Box::new(ListExecutionResult {
            providers: vec![
                (
                    ProviderType::OpenWeather,
                    describe_provider(ProviderType::OpenWeather)?,
                ),
                (
                    ProviderType::WeatherAPI,
                    describe_provider(ProviderType::WeatherAPI)?,
                ),
            ],
        }))
    }
}
