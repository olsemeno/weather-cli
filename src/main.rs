use weather_cli::config::app_config::AppConfig;
use weather_cli::logger;
use weather_cli::config::args_parser;
use weather_cli::command::command_service;
fn main() {
    let app_config = match AppConfig::from_file() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to read config file. {}", e);
            std::process::exit(1);
        }
    };
    logger::init_logger(app_config.get_logger()).unwrap();
    log::info!("Logger initialized");
    log::info!("App config: {:?}", app_config);

    let args: Vec<String> = std::env::args().collect();

    let command = match args_parser::parse_args(&args) {
        Ok(command) => command,
        Err(e) => {
            eprintln!("Failed to parse arguments. {}", e);
            std::process::exit(1);
        }
    };


    match command_service::execute_command(command) {
        Ok(result) => {
            println!("{}", result.get_printable_result());
            std::process::exit(0);
        },
        Err(e) => {
            eprintln!("Failed to execute command. {}", e);
            std::process::exit(1);
        }
    };

}















