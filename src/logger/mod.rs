use chrono::Local;
#[cfg(unix)]
use libc::{c_int, signal, SIGHUP};
use log::{LevelFilter, Log, Metadata, Record};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

//reusing my logger from another project
//ref: https://github.com/specure/nettest/tree/main/src/logger
lazy_static! {
    static ref LOGGER: Mutex<Option<Arc<FileLogger>>> = Mutex::new(None);
}

pub struct FileLogger {
    level: LevelFilter,
    log_file: Arc<Mutex<File>>,
}

impl FileLogger {
    pub fn new(level: LevelFilter, log_path: &Path) -> Result<Self, std::io::Error> {
        // Create directory if it doesn't exist
        if let Some(parent) = log_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        Ok(Self {
            level,
            log_file: Arc::new(Mutex::new(log_file)),
        })
    }

    pub fn reopen_log_file(&self, log_path: &Path) -> Result<(), std::io::Error> {
        let mut log_file = self.log_file.lock().unwrap();
        *log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;
        Ok(())
    }

    fn format_log(&self, record: &Record) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        format!("{} [{}] - {}\n", timestamp, record.level(), record.args())
    }
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = self.format_log(record);

        if let Ok(mut file) = self.log_file.lock() {
            let _ = file.write_all(message.as_bytes());
            let _ = file.flush();
        }

        // let _ = io::stdout().write_all(message.as_bytes());
        // let _ = io::stdout().flush();
    }

    fn flush(&self) {
        if let Ok(mut file) = self.log_file.lock() {
            let _ = file.flush();
        }
        let _ = io::stdout().flush();
    }
}

#[cfg(unix)]
extern "C" fn handle_sighup(_: c_int) {
    // This will be called when logrotate sends SIGHUP
    if let Some(logger) = LOGGER.lock().ok().and_then(|l| l.as_ref().cloned()) {
        let log_path = if cfg!(target_os = "macos") {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/root".to_string());
            Path::new(&home).join("Library/Logs/rmbt/weather-cli.log")
        } else {
            Path::new("/var/log/rmbt/weather-cli.log").to_path_buf()
        };
        let _ = logger.reopen_log_file(&log_path);
    }
}

pub fn init_logger(level: LevelFilter) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create log directory if it doesn't exist
    let log_dir = if cfg!(target_os = "macos") {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/root".to_string());
        Path::new(&home).join("Library/Logs/weather-cli")
    } else {
        #[cfg(unix)]
        {
            let pid = std::process::id();
            let pid_dir = Path::new("/run");
            if !pid_dir.exists() {
                std::fs::create_dir_all(pid_dir)?;
            }
            let pid_path = pid_dir.join("weather-cli.pid");
            std::fs::write(&pid_path, pid.to_string())?;
        }
        Path::new("/var/log/weather-cli").to_path_buf()
    };

    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)?;
    }

    let log_path = log_dir.join("weather-cli.log");

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;

    let logger = Arc::new(FileLogger {
        level,
        log_file: Arc::new(Mutex::new(log_file)),
    });

    *LOGGER.lock().unwrap() = Some(logger.clone());

    unsafe {
        #[cfg(unix)]
        signal(SIGHUP, handle_sighup as usize);
    }

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level);

    Ok(())
}
