use crate::config::app_config::AppConfig;
use crate::enums::ProviderType;
use crate::errors::config_error::ConfigError;
use log::LevelFilter;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

pub fn read_config_file() -> Result<AppConfig, ConfigError> {
    // Determine config file path based on OS
    let config_path = if cfg!(target_os = "macos") {
        // On macOS use ~/.config/ as main directory
        let home_dir = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(format!("{}/.config/weather-cli.conf", home_dir))
    } else {
        PathBuf::from("/etc/weather-cli.conf")
    };

    let config_content: String = if config_path.exists() {
        fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::InvalidConfig(format!("Could not read config file: {}", e)))?
    } else {
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    let error = format!("Could not create config directory: {}", e);
                    return Err(ConfigError::ConfigDirectoryNotFound(error));
                }
            }
        }
        let default_config: &str = include_str!("../../weather-cli.conf");

        if let Err(e) = fs::write(&config_path, &default_config) {
            let error = format!("Could not create config file: {}", e);
            return Err(ConfigError::InvalidConfig(error));
        }

        default_config.to_string()
    };

    let mut app_config = parse_config_content(&config_content)?;
    app_config.config_path = Some(config_path);
    Ok(app_config)
}

fn parse_config_content(content: &str) -> Result<AppConfig, ConfigError> {
    let mut config = AppConfig::default();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');

            match key {
                "provider" => {
                    config.provider = ProviderType::from_str(value)?;
                }
                "logger" => {
                    config.logger = match value {
                        "info" => LevelFilter::Info,
                        "debug" => LevelFilter::Debug,
                        "trace" => LevelFilter::Trace,
                        _ => {
                            return Err(ConfigError::InvalidConfig(format!(
                                "Unknown logger level: {}",
                                value
                            )));
                        }
                    };
                }
                _ => {
                    return Err(ConfigError::InvalidConfig(format!(
                        "Unknown config key: {}",
                        key
                    )));
                }
            }
        }
    }

    Ok(config)
}

pub fn save_config_file(app_config: &AppConfig) -> Result<(), ConfigError> {
    log::info!("Saving config file to: {:?}", app_config.config_path);
    let provider_str = app_config.provider.to_string();

    let logger_str = match app_config.logger {
        LevelFilter::Info => "info",
        LevelFilter::Debug => "debug",
        LevelFilter::Trace => "trace",
        LevelFilter::Warn => "warn",
        LevelFilter::Error => "error",
        LevelFilter::Off => "off",
    };

    let content = format!("provider={}\nlogger={}\n", provider_str, logger_str);

    match &app_config.config_path {
        Some(config_path) => {
            fs::write(&config_path, content).map_err(|e| {
                ConfigError::InvalidConfig(format!("Could not write config file: {}", e))
            })?;

            Ok(())
        }
        None => {
            return Err(ConfigError::InvalidConfig(
                "Config path is not set".to_string(),
            ));
        }
    }
}
