mod app;
mod core;
mod keys;
mod shutdown_signal;
use tokio::net::TcpListener;

use crate::app::create_app;

#[tokio::main]
async fn main() {
    let app = create_app().await;

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {}", addr));

    tracing::info!("Server listening on {}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal::shutdown_signal())
        .await
        .unwrap();
}
