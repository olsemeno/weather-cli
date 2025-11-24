use crate::enums::ProviderType;
use crate::provider::weather::Weather;
use crate::provider::provider::Provider;
use crate::provider::providers::openweather::OpenWeatherProvider;
use crate::provider::providers::weatherapi::WeatherAPIProvider;


pub fn get_weather(params: Vec<String>, provider: ProviderType) -> Result<Weather, Box<dyn std::error::Error + Send + Sync>> {
    match provider {
        ProviderType::OpenWeather => {
            let openweather_provider = OpenWeatherProvider;
            openweather_provider.get_weather(params)
        }
        ProviderType::WeatherAPI => {
            let weatherapi_provider = WeatherAPIProvider;
            weatherapi_provider.get_weather(params)
        }
    }
}