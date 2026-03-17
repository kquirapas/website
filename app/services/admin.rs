//! API Routing.
//!
//! Handles internal API responses
//!
use axum::{http::StatusCode, response::IntoResponse};

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn current_year() -> &'static str {
    "2026"
}
