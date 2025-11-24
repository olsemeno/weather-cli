use weather::config::app_config::AppConfig;
use weather::enums::ProviderType;
use weather::provider::provider_service;

fn setup_app_config_for_weatherapi() {
    let mut config = AppConfig::default();
    //have to be set from ENV variables but for now we will set it manually
    config.set_weatherapi_api_key("474206f6407b4c6f8a484643252411".to_string());
    AppConfig::update(&config).unwrap();
}

fn setup_app_config_for_openweather() {
    let mut config = AppConfig::default();
    config.set_openweather_api_key("6dc21edffd2975849ff72789865ecc66".to_string());
    AppConfig::update(&config).unwrap();
}

// ========== WeatherAPI Provider Tests ==========

#[test]
fn test_weatherapi_get_weather_single_day() {
    setup_app_config_for_weatherapi();

    let result =
        provider_service::get_weather(vec!["London".to_string()], ProviderType::WeatherAPI);
    if result.is_err() {
        eprintln!("Error: {}", result.err().unwrap());
        assert!(false, "Should return weather data");
        return;
    }
    let weathers = result.unwrap();
    assert!(
        !weathers.is_empty(),
        "Should return at least one weather entry"
    );
    let weather = &weathers[0];
    assert!(
        weather.get_temperature() > -100.0 && weather.get_temperature() < 100.0,
        "Temperature should be in reasonable range"
    );
    assert!(!weather.get_date().is_empty(), "Date should not be empty");
    println!(
        "WeatherAPI test passed: {} entries, first: {}°C on {}",
        weathers.len(),
        weather.get_temperature(),
        weather.get_date()
    );
}

#[test]
fn test_weatherapi_get_weather_multiple_days() {
    setup_app_config_for_weatherapi();

    let result = provider_service::get_weather(
        vec!["London".to_string(), "3".to_string()],
        ProviderType::WeatherAPI,
    );
    if result.is_err() {
        eprintln!("Error: {}", result.err().unwrap());
        assert!(false, "Should return weather data");
        return;
    }
    let weathers = result.unwrap();
    assert!(
        !weathers.is_empty(),
        "Should return at least one weather entry"
    );
    assert!(weathers.len() == 3, "Should return 3 weather entries");
    let weather = &weathers[0];
    assert!(
        weather.get_temperature() > -100.0 && weather.get_temperature() < 100.0,
        "Temperature should be in reasonable range"
    );
    assert!(!weather.get_date().is_empty(), "Date should not be empty");
    println!(
        "WeatherAPI test passed: {} entries, first: {}°C on {}",
        weathers.len(),
        weather.get_temperature(),
        weather.get_date()
    );
}

#[test]
fn test_weatherapi_get_weather_invalid_days() {
    setup_app_config_for_weatherapi();

    let result = provider_service::get_weather(
        vec!["London".to_string(), "0".to_string()],
        ProviderType::WeatherAPI,
    );
    if result.is_ok() {
        assert!(false, "Should return error");
        return;
    }
    assert!(result.is_err(), "Should return error");

    let result = provider_service::get_weather(
        vec!["London".to_string(), "15".to_string()],
        ProviderType::WeatherAPI,
    );
    if result.is_ok() {
        assert!(false, "Should return error");
        return;
    }
    assert!(result.is_err(), "Should return error");

    let result = provider_service::get_weather(
        vec!["London".to_string(), "1".to_string()],
        ProviderType::WeatherAPI,
    );
    if result.is_ok() {
        assert!(false, "Should return error");
        return;
    }
    assert!(result.is_err(), "Should return error");
}

// ========== OpenWeather Provider Tests ==========

#[test]
fn test_openweather_get_weather_single_day() {
    setup_app_config_for_openweather();

    let result =
        provider_service::get_weather(vec!["London".to_string()], ProviderType::OpenWeather);
    if result.is_err() {
        eprintln!("Error: {}", result.err().unwrap());
        assert!(false, "Should return weather data");
        return;
    }
    let weathers = result.unwrap();
    assert!(
        !weathers.is_empty(),
        "Should return at least one weather entry"
    );
    let weather = &weathers[0];
    assert!(
        weather.get_temperature() > -100.0 && weather.get_temperature() < 100.0,
        "Temperature should be in reasonable range"
    );
    assert!(!weather.get_date().is_empty(), "Date should not be empty");
    println!(
        "OpenWeather test passed: {} entries, first: {}°C on {}",
        weathers.len(),
        weather.get_temperature(),
        weather.get_date()
    );
}

#[test]
fn test_openweather_get_weather_multiple_days() {
    setup_app_config_for_openweather();

    let result = provider_service::get_weather(
        vec!["London".to_string(), "3".to_string()],
        ProviderType::OpenWeather,
    );
    if result.is_err() {
        eprintln!("Error: {}", result.err().unwrap());
        assert!(false, "Should return weather data");
        return;
    }
}

#[test]
fn test_openweather_get_weather_invalid_days() {
    setup_app_config_for_openweather();

    let result = provider_service::get_weather(
        vec!["London".to_string(), "0".to_string()],
        ProviderType::OpenWeather,
    );
    if result.is_ok() {
        assert!(false, "Should return error");
        return;
    }
    assert!(result.is_err(), "Should return error");

    let result = provider_service::get_weather(
        vec!["London".to_string(), "15".to_string()],
        ProviderType::OpenWeather,
    );
    if result.is_ok() {
        assert!(false, "Should return error");
        return;
    }
    assert!(result.is_err(), "Should return error");
}
