use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid config: {0}")]
    InvalidConfig(String),
    #[error("Config directory not found: {0}")]
    ConfigDirectoryNotFound(String),
    #[error("Invalid provider: {0}")]
    InvalidProvider(String),
    #[error("Provider not provided")]
    InvalidArgument(String),
}

