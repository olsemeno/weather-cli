use crate::enums::CommandType;
use crate::command::executors::configure_executor::ConfigureExecutor;
use crate::command::command_executor::ExecutionResult;
use crate::command::executors::get_executor::GetExecutor;
use crate::command::executors::list_executor::ListExecutor;

pub fn execute_command(command: CommandType) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
   match command {
        CommandType::Configure(_) => {
           ConfigureExecutor::new().execute(command)
        },
        CommandType::Get(_) => {
            GetExecutor::new().execute(command)
        }
        CommandType::List => {
            ListExecutor::new().execute(command)
        }
    }
}