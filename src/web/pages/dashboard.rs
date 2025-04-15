use axum::Router;

use crate::web::app::AppState;
use crate::web::pages::workout_schedule;

pub fn router() -> Router<AppState> {
    Router::new().nest("/workout_schedule", workout_schedule::router())
}
