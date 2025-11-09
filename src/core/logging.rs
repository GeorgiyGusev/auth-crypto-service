use tracing::Subscriber;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};

use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

pub fn setup_logging() -> impl Subscriber {
    let env_filter = EnvFilter::from_default_env();

    // tracing_subscriber::fmt()
    //     .with_env_filter(env_filter)
    //     .finish()

    let formatting_layer =
        BunyanFormattingLayer::new(env!("CARGO_CRATE_NAME").to_string(), std::io::stdout);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}
