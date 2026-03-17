//! API Routing.
//!
//! Handles internal API responses
//!
use axum::http::StatusCode;

/*
 * Note: No need to wrap these with a [`tower::Service`]
 * because these will most likely remain tightly coupled
 * with the main axum app vs other services that are bound
 * to grow as separate services on separate machines.
 */

pub async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn current_year() -> String {
    String::from("2026")
}
