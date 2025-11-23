use crate::enums::CommandType;
pub trait CommandExecutor {
    fn execute(
        &self,
        command: CommandType,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error + Send + Sync>>;
}

pub trait ExecutionResult {
    fn get_printable_result(&self) -> String;
}
