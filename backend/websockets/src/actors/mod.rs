use chrono::Utc;

use self::messages::LocationUpdateMessage;

pub mod kafka_consumer;
pub mod messages;
pub mod race;
pub mod ws;

pub fn measure_latency(msg: &LocationUpdateMessage) -> f64 {
    let now = Utc::now().timestamp_millis();
    (now - (msg.timestamp as i64 * 1000)) as f64
}
