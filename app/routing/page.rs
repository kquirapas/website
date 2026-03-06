use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

pub fn page_routing() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/software", get(software))
}

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate;

#[derive(Template)]
#[template(path = "software.html")]
struct SoftwareTemplate<'a> {
    name: &'a str,
    counter: u16,
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
