use crate::web::app::AppState;
use askama::Template;
use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(self::get::root))
}

#[derive(Template)]
#[template(path = "index.html", print = "all")]
struct RootTemplate {
    #[allow(dead_code)]
    name: String,
}

mod get {
    use axum::response::Html;

    use super::*;
    pub(crate) async fn root() -> impl IntoResponse {
        let root = RootTemplate {
            name: "Koen".into(),
        };
        Html(root.render().unwrap()).into_response()
    }
}
