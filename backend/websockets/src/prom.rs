use std::collections::HashMap;

use lazy_static::lazy_static;
use prometheus::{Histogram, IntCounter, IntGauge, Registry};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref INCOMING_REQUESTS: IntCounter =
        IntCounter::new("incoming_requests", "Incoming Requests").expect("metric can be created");
    pub static ref LOCATION_UPDATES_SENT: IntCounter =
        IntCounter::new("location_updates_sent", "Location Updates Sent")
            .expect("metric can be created");
    pub static ref LOCATION_UPDATES_RECEIVED: IntCounter =
        IntCounter::new("location_updates_received", "Location Updates Received")
            .expect("metric can be created");
    pub static ref CONNECTED_CLIENTS: IntGauge =
        IntGauge::new("connected_clients", "Connected Clients").expect("metric can be created");
    pub static ref DISCONNECTED_CLIENTS: IntGauge =
        IntGauge::new("disconnected_clients", "Disconnected Clients")
            .expect("metric can be created");
    pub static ref LOCATION_UPDATE_LATENCY: Histogram =
        Histogram::with_opts(prometheus::HistogramOpts {
            buckets: prometheus::linear_buckets(5.0, 5.0, 400).unwrap(),
            common_opts: prometheus::Opts {
                namespace: "websockets".to_string(),
                subsystem: "location_update".to_string(),
                name: "latency_milliseconds".to_string(),
                help: "Location update latency".to_string(),
                const_labels: HashMap::from([("env".to_string(), "local".to_string())]),
                variable_labels: vec![],
            }
        })
        .expect("metric can be created");
}

pub fn register_custom_metrics() {
    REGISTRY
        .register(Box::new(INCOMING_REQUESTS.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(CONNECTED_CLIENTS.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(DISCONNECTED_CLIENTS.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(LOCATION_UPDATES_SENT.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(LOCATION_UPDATES_RECEIVED.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(LOCATION_UPDATE_LATENCY.clone()))
        .expect("collector can be registered");
}
