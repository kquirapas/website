//! API Routing.
//!
//! Handles internal API responses and external API middleware.
//!
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

async fn current_year() -> &'static str {
    "2026"
}
