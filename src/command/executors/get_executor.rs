use crate::command::command_executor::CommandExecutor;
use crate::command::command_executor::ExecutionResult;
use crate::config::app_config::AppConfig;
use crate::enums::CommandType;
use crate::errors::execution_error::ExecutionError;
use crate::provider::provider_service::get_weather;
use crate::provider::weather::Weather;
pub struct GetExecutor;

pub struct GetExecutionResult {
    pub weathers: Vec<Weather>,
}

impl ExecutionResult for GetExecutionResult {
    fn get_printable_result(&self) -> String {
        format!(
            "Weather: \n{} ",
            self.weathers
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl CommandExecutor for GetExecutor {
    fn execute(
        &self,
        command: CommandType,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        match command {
            CommandType::Get(params) => self.get_weather(params),
            _ => Err(Box::new(ExecutionError::InvalidCommand(
                format!("{}", command),
            ))),
        }
    }
}

impl GetExecutor {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<dyn CommandExecutor> {
        Box::new(GetExecutor)
    }
    fn get_weather(
        &self,
        params: Vec<String>,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        let app_config = AppConfig::get().ok_or("App config not found")?;
        let weather = get_weather(params, app_config.get_provider())?;
        Ok(Box::new(GetExecutionResult { weathers: weather }))
    }
}
