//! HTTP route handlers for the UpdrageGuard API.

use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn root() -> Json<Value> {
    Json(json!({
        "name": "UpdrageGuard API",
        "description": "Soroban Smart Contract Upgrade & Migration Safety Platform",
        "status": "ok",
    }))
}

pub async fn health() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({ "status": "healthy" })),
    )
}
