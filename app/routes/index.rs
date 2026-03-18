use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> String {
    String::from("Hello, World!")
}
