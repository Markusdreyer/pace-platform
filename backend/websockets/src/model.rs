use serde::Deserialize;

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
