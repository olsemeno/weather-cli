use crate::enums::ProviderType;
use crate::provider::provider::Provider;
use crate::provider::providers::openweather::OpenWeatherProvider;
use crate::provider::providers::weatherapi::WeatherAPIProvider;
use crate::provider::weather::Weather;

pub fn get_weather(
    params: Vec<String>,
    provider: ProviderType,
) -> Result<Vec<Weather>, Box<dyn std::error::Error + Send + Sync>> {
    let provider = get_provider(provider);
    provider.get_weather(params)
}

pub fn describe_provider(
    provider: ProviderType,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let provider = get_provider(provider);
    provider.describe()
}

fn get_provider(provider: ProviderType) -> Box<dyn Provider> {
    match provider {
        ProviderType::OpenWeather => Box::new(OpenWeatherProvider),
        ProviderType::WeatherAPI => Box::new(WeatherAPIProvider),
    }
}
