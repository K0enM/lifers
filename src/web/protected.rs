use axum::Router;
use axum::routing::get;

#[allow(dead_code)]
type AuthSession = axum_login::AuthSession<Backend>;
use crate::users::Backend;

use super::app::AppState;
use axum::{http::StatusCode, response::IntoResponse};
use axum_messages::Messages;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(self::get::protected))
}

mod get {
    use super::*;

    pub async fn protected(auth_session: AuthSession, messages: Messages) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => user.username.into_response(),
            None => StatusCode::UNAUTHORIZED.into_response(),
        }
    }
}
