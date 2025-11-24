use crate::config::app_config::AppConfig;
use crate::errors::provider_error::ProviderError;
use crate::provider::provider::Provider;
use crate::provider::weather::{Weather, WeatherBuilder};
use serde::Deserialize;

pub struct OpenWeatherProvider;

#[derive(Deserialize)]
struct OpenWeatherResponse {
    main: Main,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
}

impl Provider for OpenWeatherProvider {
    fn get_weather(
        &self,
        params: Vec<String>,
    ) -> Result<Weather, Box<dyn std::error::Error + Send + Sync>> {
        if params.is_empty() {
            return Err(ProviderError::AddressRequired("City name is required".into()).into());
        }

        let _ = self.validate_date_param(&params);

        let city = &params[0];
        let app_config =
            AppConfig::get().ok_or(ProviderError::APIKeyRequired("App config not found".into()))?;

        let api_key = app_config
            .get_openweather_api_key()
            .ok_or(ProviderError::APIKeyRequired(
                "OpenWeather API key is required".into(),
            ))?;

        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?id={}&appid={}",
            city, api_key
        );

        log::info!("OpenWeather API URL: {}", url);

        let response = reqwest::blocking::get(&url)?;

        if !response.status().is_success() {
            return Err(ProviderError::APIError(format!(
                "OpenWeather API error: {}",
                response.status()
            ))
            .into());
        }

        let weather_data: OpenWeatherResponse = response.json()?;

        WeatherBuilder::new()
            .temperature(weather_data.main.temp)
            .build()
            .map_err(|e| e.into())
    }
}
