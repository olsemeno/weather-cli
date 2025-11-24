use crate::provider::weather::Weather;

pub trait Provider {
    fn get_weather(
        &self,
        params: Vec<String>,
    ) -> Result<Weather, Box<dyn std::error::Error + Send + Sync>>;

    fn validate_date_param(&self, params: &[String]) -> Option<chrono::NaiveDate> {
        if params.len() > 2 {
            let last_param = params.last()?;
            dateparser::parse(last_param).ok().map(|dt| dt.date_naive())
        } else {
            None
        }
    }

}
