use std::{sync::Arc, time::Duration};

use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    connections::Connections,
    keys::{init_keys_router, router::ApiDoc},
};

pub mod connections;
mod helpers;
pub mod keys;
mod logging;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Инициализация логирования
    logging::setup_logging().init();

    tracing::info!("Starting auth-crypto-service...");

    let vault_client = helpers::init_vault_client().expect("Failed to initialize Vault client");

    let connections = Arc::new(Connections {
        vault_conn: vault_client,
    });

    let keys_router = init_keys_router(connections)
        .await
        .expect("failed to initialize keys router");

    let app = Router::new()
        .nest("/auth-crypto-service/api/v1", keys_router)
        .merge(SwaggerUi::new("/auth-crypto-service/api/v1/docs").url(
            "/auth-crypto-service/api/v1/api-docs/openapi.json",
            ApiDoc::openapi(),
        ))
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(TraceLayer::new_for_http());

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {}", addr));

    tracing::info!("Server listening on {}", addr);
    tracing::info!("Swagger UI available at http://{}/docs", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received CTRL+C signal, shutting down gracefully...");
        },
        _ = terminate => {
            tracing::info!("Received TERM signal, shutting down gracefully...");
        },
    }

    tracing::info!("Shutdown completed");
}
