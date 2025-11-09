use std::time::Duration;

use axum::Router;
use dotenvy::dotenv;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::util::SubscriberInitExt;

use super::{core, keys};

// TODO: Add endpoint to serve hand-written OpenAPI spec

pub async fn create_app() -> Router {
    dotenv().ok();

    core::logging::setup_logging().init();

    tracing::info!("Starting auth-crypto-service...");

    let vault_client =
        core::helpers::init_vault_client().expect("Failed to initialize Vault client");

    let keys_router = keys::init_keys_router(vault_client)
        .await
        .expect("failed to initialize keys router");

    let cors = CorsLayer::permissive();

    Router::new()
        .nest("/auth-crypto-service/api/v1", keys_router)
        .layer(cors)
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(TraceLayer::new_for_http())
}
