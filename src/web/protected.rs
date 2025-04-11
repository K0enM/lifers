use askama::Template;
use axum::Router;
use axum::routing::get;

#[allow(dead_code)]
type AuthSession = axum_login::AuthSession<Backend>;
use crate::users::Backend;

use super::app::AppState;
use axum::{http::StatusCode, response::IntoResponse};
use axum_messages::Messages;

pub fn router() -> Router<AppState> {
    Router::new().route("/app/{path}", get(self::get::protected))
}

#[derive(Template)]
#[template(path = "app.html")]
struct AppTemplate {
    username: String,
    path: String,
}

mod get {
    use axum::{
        extract::{Path, State},
        response::Html,
    };

    use super::*;

    #[axum::debug_handler]
    pub async fn protected(
        Path(path): Path<String>,
        auth_session: AuthSession,
        _messages: Messages,
        State(_state): State<AppState>,
    ) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => Html(
                AppTemplate {
                    username: user.username,
                    path,
                }
                .render()
                .unwrap(),
            )
            .into_response(),
            None => StatusCode::UNAUTHORIZED.into_response(),
        }
    }
}
