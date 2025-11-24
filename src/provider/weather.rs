pub struct Weather {
    temperature: f64,
}

impl Weather {
    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }
}

impl std::fmt::Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Temperature: {:.1}°C", self.temperature)
    }
}

pub struct WeatherBuilder {
    temperature: Option<f64>,
}

impl WeatherBuilder {
    pub fn new() -> Self {
        WeatherBuilder {
            temperature: None,
        }
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn build(self) -> Result<Weather, String> {
        let temperature = self.temperature.ok_or("Temperature is required")?;
        Ok(Weather { temperature })
    }
}
