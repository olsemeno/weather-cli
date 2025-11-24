use crate::errors::provider_error::ProviderError;
pub struct Weather {
    temperature: f64,
    date: String,
}

impl Weather {
    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }
    pub fn get_date(&self) -> &String {
        &self.date
    }
}

impl std::fmt::Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Temperature: {:.1}°C, Date: {}",
            self.temperature, self.date
        )
    }
}

pub struct WeatherBuilder {
    temperature: Option<f64>,
    date: Option<String>,
}

impl WeatherBuilder {
    pub fn new() -> Self {
        WeatherBuilder {
            temperature: None,
            date: None,
        }
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn date(mut self, date: String) -> Self {
        self.date = Some(date);
        self
    }
    pub fn build(self) -> Result<Weather, Box<dyn std::error::Error + Send + Sync>> {
        let temperature = self.temperature.ok_or(ProviderError::TemperatureRequired(
            "Temperature is required".into(),
        ))?;
        let date = self
            .date
            .ok_or(ProviderError::DateRequired("Date is required".into()))?;
        Ok(Weather { temperature, date })
    }
}
