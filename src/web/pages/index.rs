use crate::web::app::AppState;
use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(self::get::root))
}

mod get {
    use super::*;
    pub(crate) async fn root() -> impl IntoResponse {}
}
