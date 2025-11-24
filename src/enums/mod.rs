use crate::errors::config_error::ConfigError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderType {
    OpenWeather,
    WeatherAPI,
}

impl FromStr for ProviderType {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openweather" => Ok(ProviderType::OpenWeather),
            "weatherapi" => Ok(ProviderType::WeatherAPI),
            _ => Err(ConfigError::InvalidProvider(s.to_string())),
        }
    }
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::OpenWeather => write!(f, "OpenWeather"),
            ProviderType::WeatherAPI => write!(f, "WeatherAPI"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommandType {
    Configure(ProviderType),
    Get(Vec<String>),
    List,
}

impl std::fmt::Display for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandType::Configure(provider) => write!(f, "Configure({})", provider),
            CommandType::Get(_) => write!(f, "Get"),
            CommandType::List => write!(f, "List"),
        }
    }
}
