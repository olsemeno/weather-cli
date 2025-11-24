use crate::provider::weather::Weather;

pub trait Provider {
    fn get_weather(
        &self,
        params: Vec<String>,
    ) -> Result<Vec<Weather>, Box<dyn std::error::Error + Send + Sync>>;

    fn describe(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    //if the last parameter is an integer, return it for forecast days, otherwise return None
    fn validate_date_param(&self, params: &[String]) -> Option<i32> {
        log::debug!("Validating date parameter: {:?}", params);
        if params.len() > 1 {
            log::debug!("Last parameter: {:?}", params.last());
            let last_param = params.last()?;
            if last_param.parse::<i32>().is_ok() {
                log::debug!("Last parameter is an integer: {:?}", last_param.parse::<i32>().unwrap());
                Some(last_param.parse::<i32>().unwrap())
            } else {
                None
            }
        } else {
            None
        }
    }

}
