use tracing_subscriber::{prelude::*, EnvFilter};

pub fn configure_log(level: String) {
    let env_filter = EnvFilter::new(level);
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
