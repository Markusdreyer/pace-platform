use serde_json::{json, Value};
use std::{env, string::FromUtf8Error};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum WebSocketError {
    #[error("invalid websocket message")]
    InvalidMessage(String),
    #[error("could not parse data: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("uf8 error: {0}")]
    Utf8Error(#[from] FromUtf8Error),
}

impl WebSocketError {
    pub fn to_json(&self) -> Value {
        match self {
            WebSocketError::InvalidMessage(msg) => json!({
                "error": "Invalid Message",
                "reason": msg
            }),
            WebSocketError::SerdeError(err) => json!({
                "error": "Serde Error",
                "reason": format!("{err}")
            }),
            WebSocketError::Utf8Error(err) => json!({
                "error": "Utf8 Error",
                "reason": format!("{err}")
            }),
        }
    }
}

pub trait ErrorToJson {
    fn error_type(&self) -> &'static str;
    fn error_reason(&self) -> String;

    fn to_json(&self) -> Value {
        json!({
            "error": self.error_type(),
            "reason": self.error_reason()
        })
    }
}

impl ErrorToJson for WebSocketError {
    fn error_type(&self) -> &'static str {
        match self {
            WebSocketError::InvalidMessage(_) => "Invalid Message",
            WebSocketError::SerdeError(_) => "Serde Error",
            WebSocketError::Utf8Error(_) => "Utf8 Error",
        }
    }

    fn error_reason(&self) -> String {
        match self {
            WebSocketError::InvalidMessage(msg) => msg.clone(),
            WebSocketError::SerdeError(err) => format!("{err}"),
            WebSocketError::Utf8Error(err) => format!("{err}"),
        }
    }
}
