use crate::web::app::AppState;
use crate::web::entities::users::User;
use askama::Template;
use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(self::get::root))
}

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate {}

#[derive(Template)]
#[template(path = "app.html")]
struct AppTemplate {
    user: User,
}

mod get {
    use axum::{extract::State, response::Html};
    use axum_messages::Messages;

    use super::*;
    use crate::web::auth::AuthSession;

    pub async fn root(
        auth_session: AuthSession,
        _messages: Messages,
        State(_state): State<AppState>,
    ) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => Html(
                AppTemplate {
                    user,
                }
                .render()
                .unwrap(),
            )
            .into_response(),
            None => {
                let root = RootTemplate {};
                Html(root.render().unwrap()).into_response()
            }
        }
    }
}
