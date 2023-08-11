use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    #[serde(default = "default_log")]
    pub log: Log,
    #[serde(default = "default_kafka")]
    pub kafka: Kafka,
}

fn default_kafka() -> Kafka {
    Kafka {
        host: "localhost:9092".into(),
        auto_commit: true,
        security_protocol: "PLAINTEXT".into(),
        topics: Topics {
            location_update: "location_update".into(),
        },
    }
}

#[derive(Deserialize)]
pub struct Kafka {
    pub host: String,
    pub auto_commit: bool,
    pub security_protocol: String,
    pub topics: Topics,
}

#[derive(Deserialize, Clone)]
pub struct Topics {
    pub location_update: String,
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
