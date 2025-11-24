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

## CI/CD Workflows

### CI Workflow (`ci.yml`)
- Runs on every push and pull request to `main`
- Checks code formatting with `rustfmt`
- Runs `clippy` for code quality
- Builds the project
- Runs integration tests with real API calls

### Release Workflow (`release.yml`)
- Manual workflow for creating releases
- Allows selecting branch and version (e.g., `0.0.1`)
- Creates git tag `v{version}`
- Builds release binaries for macOS and Ubuntu
- Creates GitHub Release with downloadable artifacts
