mod config;
mod routing;

use anyhow::Context;
use axum::Router;
use routing::{api, page};
use tower_http::services::ServeDir;

use crate::config::Config;

pub async fn run_service() -> anyhow::Result<()> {
    let config = Config::default();

    // build our application with a single route
    let port = config.port;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let mut asset_dir = config.base_directory;
    asset_dir.push("/public");

    let app = Router::new()
        .nest("/api", api::api_routing())
        .nest("/", page::page_routing())
        .nest_service("/public", ServeDir::new(asset_dir));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .context("error while starting server")?;

    Ok(())
}
