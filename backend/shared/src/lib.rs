use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

pub mod log;

pub fn setup_config<'a, T: Deserialize<'a>>() -> Result<T, ConfigError> {
    let env = env::var("RUN_MODE").unwrap_or_else(|_| "local".into());
    let config = Config::builder()
        .add_source(File::with_name("config/default").required(false))
        .add_source(File::with_name(&format!("config/{env}")).required(false))
        .add_source(Environment::default().separator("__"))
        .build()?;
    config.try_deserialize()
}
