use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid config: {0}")]
    InvalidConfig(String),
    #[error("Config directory not found: {0}")]
    ConfigDirectoryNotFound(String),
    #[error("Invalid provider: {0}")]
    InvalidProvider(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("API key is required: {0}")]
    APIKeyRequired(String),
}
