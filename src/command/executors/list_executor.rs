use crate::command::command_executor::ExecutionResult;
use crate::command::command_executor::CommandExecutor;
use crate::enums::CommandType;
use crate::enums::ProviderType;
use crate::errors::execution_error::ExecutionError;

pub struct ListExecutor;

pub struct ListExecutionResult {
    pub providers: Vec<ProviderType>,
}

impl ExecutionResult for ListExecutionResult {
    fn get_printable_result(&self) -> String {
        format!("Providers: {}", self.providers.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", "))
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
                return Err(Box::new(ExecutionError::InvalidCommand(command.to_string())))
            }
        }
    }
}

impl ListExecutor {
    pub fn new() -> Box<dyn CommandExecutor> {
        Box::new(ListExecutor)
    }
    fn list_providers(&self) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Box::new(ListExecutionResult { providers: vec![ProviderType::OpenWeather, ProviderType::WeatherAPI] }))
    }
}