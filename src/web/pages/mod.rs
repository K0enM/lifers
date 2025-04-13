use axum::Router;

use super::app::AppState;

pub mod dashboard;
pub mod index;

pub fn router() -> Router<AppState> {
    index::router().merge(dashboard::router())
}
