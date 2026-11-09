//! UpdrageGuard backend API.
//!
//! A small Axum service exposing endpoints to analyze contract upgrades.
//! The heavy lifting is performed by the engine crates; this binary only
//! wires up routing, tracing, and JSON serialization.

use std::net::SocketAddr;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/", get(routes::root))
        .route("/health", get(routes::health))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("UpdrageGuard API listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
