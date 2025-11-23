use crate::enums::CommandType;
use crate::command::executors::configure_executor::ConfigureExecutor;
use crate::command::command_executor::ExecutionResult;

pub fn execute_command(command: CommandType) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
   match command {
        CommandType::Configure(_) => {
           ConfigureExecutor::new().execute(command)
        },
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid command",
            )))
        }
    }
}