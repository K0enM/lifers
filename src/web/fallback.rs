use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Debug, Template)]
#[template(path = "fallback.html")]
struct FallbackTemplate {}

pub async fn handler_404() -> impl IntoResponse {
    let template = FallbackTemplate {};
    (StatusCode::NOT_FOUND, Html(template.render().unwrap())).into_response()
}
