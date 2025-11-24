# Weather CLI

A command-line tool for fetching weather forecasts from multiple providers (OpenWeather and WeatherAPI).

## Purpose

Weather CLI allows you to get weather information for any city directly from your terminal. It supports multiple weather providers and can fetch forecasts for up to 14 days.

## Usage

```bash
# Get current weather for a city
weather get London

# Get weather forecast for multiple days
weather get London 3

# List available providers
weather list

# Configure a provider
weather configure openweather

# Show help
weather help
```

## Running Tests

```bash
# Run all tests
cargo test

# Run integration tests with real API calls
cargo test --test api_integration_test

```

## Download

Download the latest release from [GitHub Releases](https://github.com/olsemeno/weather-cli/releases):

- **macOS**: [weather-*-macos.tar.gz](https://github.com/olsemeno/weather-cli/releases/latest)
- **Ubuntu**: [weather-*-ubuntu.tar.gz](https://github.com/olsemeno/weather-cli/releases/latest)
