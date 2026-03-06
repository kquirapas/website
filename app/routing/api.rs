//! API Routing.
//!
//! Handles internal API responses and external API middleware.
//!

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn api_routing() -> Router {
    Router::new()
        .route("/", get(health))
        .route("/year", get(current_year))
}

/// Returns the health of all api sub route.
///
/// - /internal
async fn health() -> impl IntoResponse {
    StatusCode::OK
}

async fn current_year() -> &'static str {
    "2026"
}
