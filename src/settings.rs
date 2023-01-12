use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;

lazy_static! {
  pub static ref SETTINGS: Settings = Settings::new().expect("Failed to setup settings");
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
  pub level: String,
  pub log_path: String,
  pub log_file_prefix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
  pub uri: String,
  pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
  pub environment: String,
  pub logger: Logger,
  pub database: Database,
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

    let builder = Config::builder()
      .add_source(File::with_name("config/default"))
      .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
      .add_source(Environment::default().separator("__"));

    builder
      .build()?
      .try_deserialize()
  }
}
