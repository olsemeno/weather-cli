use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
}
