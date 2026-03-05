mod config;

use anyhow::Context;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate;

#[derive(Template)]
#[template(path = "software.html")]
struct SoftwareTemplate<'a> {
    name: &'a str,
    counter: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::default();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // let html_string = index.render().unwrap();

    info!("initializing router...");
    // build our application with a single route
    let public_path = std::env::current_dir().unwrap();
    let port = config.port;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let api_router = Router::new()
        .route("/", get(health))
        .route("/year", get(current_year));

    let page_router = Router::new()
        .route("/", get(index))
        .route("/software", get(software));

    let app = Router::new()
        .nest("/api", api_router)
        .nest("/", page_router)
        .nest_service(
            "/public",
            ServeDir::new(format!("{}/public", public_path.to_str().unwrap())),
        );

    // run our app with hyper, listening globally on port 3000
    info!("router initialized, now listening on port {}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .context("Error while starting server")?;

    Ok(())
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

async fn index() -> impl IntoResponse {
    let template = BaseTemplate;
    HtmlTemplate(template)
}

async fn software() -> impl IntoResponse {
    let template = SoftwareTemplate {
        name: "world",
        counter: 0,
    };
    HtmlTemplate(template)
}

async fn current_year() -> &'static str {
    "2026"
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
