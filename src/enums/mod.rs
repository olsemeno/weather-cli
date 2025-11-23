use std::str::FromStr;
use crate::errors::config_error::ConfigError;

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

impl ToString for ProviderType {
    fn to_string(&self) -> String {
        match self {
            ProviderType::OpenWeather => "OpenWeather".to_string(),
            ProviderType::WeatherAPI => "WeatherAPI".to_string(),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandType {
  Configure(ProviderType),
  Get,
}