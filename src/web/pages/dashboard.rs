use axum::Router;

use crate::web::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
}

mod get {}
