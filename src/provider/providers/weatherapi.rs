use crate::provider::provider::Provider;
use crate::provider::weather::Weather;

pub struct WeatherAPIProvider;

impl Provider for WeatherAPIProvider {
    fn get_weather(&self, _params: Vec<String>) -> Result<Weather, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement WeatherAPI provider
        Err("WeatherAPI provider not yet implemented".into())
    }
}

