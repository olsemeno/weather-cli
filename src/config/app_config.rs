use crate::enums::ProviderType;
use crate::errors::config_error::ConfigError;
use lazy_static::lazy_static;
use log::LevelFilter;
use std::sync::{Arc, Mutex};
use crate::config::config_file_parser::read_config_file;
use crate::config::config_file_parser::save_config_file;
use std::path::PathBuf;

lazy_static! {
    pub static ref APP_CONFIG: Arc<Mutex<Option<AppConfig>>> = Arc::new(Mutex::new(None));
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    config_path: Option<PathBuf>,
    logger: LevelFilter,
    provider: ProviderType,
    openweather_api_key: Option<String>,
    weatherapi_api_key: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            config_path: Option::None,
            logger: LevelFilter::Info,
            provider: ProviderType::OpenWeather,
            openweather_api_key: None,
            weatherapi_api_key: None,
        }
    }
}

impl AppConfig {
    pub fn from_file() -> Result<AppConfig, ConfigError> {
        let config = read_config_file();
        match config {
            Ok(config) => {
                let mut global_config = APP_CONFIG.lock().unwrap();
                *global_config = Some(config.clone());
                Ok(config)
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn update(app_config: &AppConfig) -> Result<(), ConfigError> {
        let mut global_config = APP_CONFIG.lock().unwrap();
        *global_config = Some(app_config.clone());
        Ok(())
    }

    pub fn get() -> Option<AppConfig> {
        APP_CONFIG.lock().unwrap().clone()
    }

    pub fn rewrite_config_file(&self) -> Result<(), ConfigError> {
        save_config_file(&self)
    }

    pub fn get_logger(&self) -> LevelFilter {
        self.logger
    }

    pub fn get_provider(&self) -> ProviderType {
        self.provider
    }

    pub fn get_config_path(&self) -> &Option<PathBuf> {
        &self.config_path
    }

    pub fn set_logger(&mut self, logger: LevelFilter) {
        self.logger = logger;
    }

    pub fn set_provider(&mut self, provider: ProviderType) {
        self.provider = provider;
    }

    pub fn set_config_path(&mut self, config_path: Option<PathBuf>) {
        self.config_path = config_path;
    }

    pub fn get_openweather_api_key(&self) -> Option<&String> {
        self.openweather_api_key.as_ref()
    }

    pub fn get_weatherapi_api_key(&self) -> Option<&String> {
        self.weatherapi_api_key.as_ref()
    }

    pub fn set_openweather_api_key(&mut self, openweather_api_key: String) {
        self.openweather_api_key = Some(openweather_api_key);
    }

    pub fn set_weatherapi_api_key(&mut self, weatherapi_api_key: String) {
        self.weatherapi_api_key = Some(weatherapi_api_key);
    }

}
