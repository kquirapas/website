use crate::services::admin;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(admin::health))
        .route("/year", get(admin::current_year))
}
