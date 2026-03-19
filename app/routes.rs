use crate::services::admin;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        // Index.
        .route("/", get(hello_world))
        // Admin.
        .route("/health", get(admin::health))
        .route("/year", get(admin::current_year))
}

async fn hello_world() -> String {
    String::from("Hello, World!")
}
