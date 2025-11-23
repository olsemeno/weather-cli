use log::LevelFilter;

#[derive(Debug, Clone)]
pub struct AppConfig {
  pub logger: LevelFilter,
}

impl Default for AppConfig {
  fn default() -> Self {
    Self {
      logger: LevelFilter::Info,
    }
  }
}
