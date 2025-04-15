use askama::Template;
use axum::{
    Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use axum_messages::Messages;

use crate::web::{app::AppState, auth::AuthSession};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/edit", get(edit))
        .route("/edit", post(edit))
        .route("/delete", post(delete))
}

#[derive(Debug, Template)]
#[template(path = "workout_schedule/list.html")]
struct ListTemplate {}
async fn list(
    auth_session: AuthSession,
    _messages: Messages,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    let user = auth_session.user.unwrap();
    "Hello, world".into_response()
}

#[derive(Debug, Template)]
#[template(path = "workout_schedule/create.html")]
struct CreateTemplate {}

async fn create(
    auth_session: AuthSession,
    _messages: Messages,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    "Hello, world".into_response()
}

#[derive(Debug, Template)]
#[template(path = "workout_schedule/edit.html")]
struct EditTemplate {}

async fn edit(
    auth_session: AuthSession,
    _messages: Messages,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    "Hello, world".into_response()
}

async fn delete(
    auth_session: AuthSession,
    _messages: Messages,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    "Hello, world".into_response()
}
