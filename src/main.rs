use weather_cli::config::parser::read_config_file;
use weather_cli::logger;

fn main() {
    let app_config = read_config_file();
    if let Err(e) = app_config {
        eprintln!("Failed to read config file: {}", e);
        std::process::exit(1);
    } else {
        let app_config = app_config.unwrap();
        logger::init_logger(app_config.logger).unwrap();
        log::info!("Logger initialized");
        log::info!("App config: {:?}", app_config);
        
    }
}
