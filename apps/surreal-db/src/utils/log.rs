use serde::Deserialize;
use tracing_subscriber::{prelude::*, EnvFilter};

use std::env;

use config::{Config, ConfigError, Environment, File};

pub fn setup_config<'a, T: Deserialize<'a>>() -> Result<T, ConfigError> {
    let env = env::var("RUN_MODE").unwrap_or_else(|_| "local".into());
    let config = Config::builder()
        .add_source(File::with_name("config/default").required(false))
        .add_source(File::with_name(&format!("config/{env}")).required(false))
        .add_source(Environment::default().separator("__"))
        .build()?;
    config.try_deserialize()
}

pub fn configure_log(level: String) {
    let env_filter = EnvFilter::new(level);
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[derive(Deserialize)]
pub struct Settings {
    #[serde(default = "default_log")]
    pub log: Log,
}

#[derive(Deserialize)]
pub struct Log {
    #[serde(default = "default_level")]
    pub level: String,
    #[serde(default = "default_pretty_print")]
    pub pretty_print: bool,
}

fn default_log() -> Log {
    Log {
        level: default_level(),
        pretty_print: default_pretty_print(),
    }
}

fn default_level() -> String {
    "info".into()
}

fn default_pretty_print() -> bool {
    false
}
