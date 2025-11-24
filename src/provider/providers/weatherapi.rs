use crate::config::app_config::AppConfig;
use crate::errors::provider_error::ProviderError;
use crate::provider::provider::Provider;
use crate::provider::weather::Weather;
use crate::provider::weather::WeatherBuilder;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherAPIResponse {
    forecast: Forecast,
}

#[derive(Deserialize, Debug)]
struct Forecast {
    forecastday: Vec<ForecastDay>,
}

#[derive(Deserialize, Debug)]
struct ForecastDay {
    date: String,
    day: Day,
}

#[derive(Deserialize, Debug)]
struct Day {
    maxtemp_c: f64,
}

pub struct WeatherAPIProvider;

impl Provider for WeatherAPIProvider {
    fn get_weather(
        &self,
        params: Vec<String>,
    ) -> Result<Vec<Weather>, Box<dyn std::error::Error + Send + Sync>> {
        if params.is_empty() {
            return Err(ProviderError::AddressRequired("City name is required".into()).into());
        }

        let date_param = self.validate_date_param(&params);

        let city = if let Some(_) = date_param {
            params[..params.len() - 1].join(" ")
        } else {
            params.join(" ")
        };

        let days = date_param.unwrap_or(1);

        if days > 14 || days < 1 {
            return Err(ProviderError::APIError(
                "Forecast days cannot be more than 14 and less than 1".into(),
            )
            .into());
        }

        let app_config =
            AppConfig::get().ok_or(ProviderError::APIKeyRequired("App config not found".into()))?;

        let api_key = app_config
            .get_weatherapi_api_key()
            .ok_or(ProviderError::APIKeyRequired(
                "WeatherAPI API key is required".into(),
            ))?;

        let url = format!(
            "https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days={}",
            api_key, city, days
        );

        log::info!("WeatherAPI API URL: {}", url);

        let response = reqwest::blocking::get(&url)?;
        if !response.status().is_success() {
            return Err(ProviderError::APIError(format!(
                "WeatherAPI API error: {}",
                response.status()
            ))
            .into());
        }

        let weather_data: WeatherAPIResponse = response.json()?;

        log::debug!("WeatherAPI API response: {:?}", weather_data);

        if weather_data.forecast.forecastday.is_empty() {
            return Err(ProviderError::APIError("No forecast data available".into()).into());
        }

        let mut weathers = Vec::new();
        for forecast_day in weather_data.forecast.forecastday {
            let weather = WeatherBuilder::new()
                .temperature(forecast_day.day.maxtemp_c)
                .date(forecast_day.date)
                .build()
                .map_err(|e| ProviderError::APIError(e.to_string()))?;
            weathers.push(weather);
        }
        Ok(weathers)
    }

    fn describe(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("WeatherAPI provider. \nUse city name or city id instead of city name.\nSupports date parameter. Days range: 1-14".to_string())
    }
}
