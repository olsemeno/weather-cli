use crate::config::app_config::AppConfig;
use crate::errors::provider_error::ProviderError;
use crate::provider::Provider;
use crate::provider::weather::{Weather, WeatherBuilder};
use serde::Deserialize;
use std::collections::HashSet;

pub struct OpenWeatherProvider;

#[derive(Deserialize, Debug)]
struct OpenWeatherForecastResponse {
    list: Vec<ForecastItem>,
}

#[derive(Deserialize, Debug)]
struct ForecastItem {
    #[allow(unused)]
    dt: i64,
    main: Main,
    dt_txt: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    #[allow(unused)]
    temp: f64,
    temp_max: f64,
    #[allow(unused)]
    temp_min: f64,
}

impl Provider for OpenWeatherProvider {
    fn get_weather(
        &self,
        params: Vec<String>,
    ) -> Result<Vec<Weather>, Box<dyn std::error::Error + Send + Sync>> {
        if params.is_empty() {
            return Err(ProviderError::AddressRequired("City name is required".into()).into());
        }

        let date_param = self.validate_date_param(&params);

        let city = if date_param.is_some() {
            params[..params.len() - 1].join(" ")
        } else {
            params.join(" ")
        };

        let days = date_param.unwrap_or(1);

        if !(1..=5).contains(&days) {
            return Err(ProviderError::APIError(
                "Forecast days cannot be more than 5 and less than 1".into(),
            )
            .into());
        }

        let app_config =
            AppConfig::get().ok_or(ProviderError::APIKeyRequired("App config not found".into()))?;

        let api_key = app_config
            .get_openweather_api_key()
            .ok_or(ProviderError::APIKeyRequired(
                "OpenWeather API key is required".into(),
            ))?;

        let url = format!(
            "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=metric",
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

        let weather_data: OpenWeatherForecastResponse = response.json()?;

        log::debug!("OpenWeather API response: {:?}", weather_data);

        if weather_data.list.is_empty() {
            return Err(ProviderError::APIError("No forecast data available".into()).into());
        }

        let mut weathers = Vec::new();
        let mut processed_dates = HashSet::new();

        for forecast_item in weather_data.list {
            let date = forecast_item
                .dt_txt
                .split_whitespace()
                .next()
                .unwrap_or(&forecast_item.dt_txt)
                .to_string();

            if !processed_dates.contains(&date) && weathers.len() < days as usize {
                let weather = WeatherBuilder::new()
                    .temperature(forecast_item.main.temp_max)
                    .date(date.clone())
                    .build()
                    .map_err(|e| ProviderError::APIError(e.to_string()))?;
                weathers.push(weather);
                processed_dates.insert(date);
            }
        }
        Ok(weathers)
    }

    fn describe(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("OpenWeather provider.\nUse city name or city id instead of city name.\nSupports date parameter. Days range: 1-5 (free tier)".to_string())
    }
}
