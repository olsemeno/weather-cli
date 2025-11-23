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
    pub config_path: Option<PathBuf>,
    pub logger: LevelFilter,
    pub provider: ProviderType,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            config_path: Option::None,
            logger: LevelFilter::Info,
            provider: ProviderType::OpenWeather,
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
    
}
