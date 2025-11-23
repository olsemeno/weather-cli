#[derive(Debug, Clone)]
pub enum ConfigError {
    InvalidConfig(String),
    ConfigDirectoryNotFound(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config error: {}", self)
    }
}
