use crate::web::app::AppState;
use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
}

mod post {

    use super::*;

    pub async fn login(State(state): State<AppState>) -> impl IntoResponse {
        "yeet".into_response()
    }
}

mod get {
    use super::*;

    pub async fn login(State(state): State<AppState>) -> impl IntoResponse {
        "yeet".into_response()
    }

    pub async fn logout(State(state): State<AppState>) -> impl IntoResponse {
        "yeet".into_response()
    }
}
