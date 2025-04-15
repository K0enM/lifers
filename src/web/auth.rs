use crate::web::app::AppState;
use crate::web::entities::users::Backend;
use crate::web::entities::users::CreateUser;
use crate::web::entities::users::Credentials;
use crate::web::entities::users::User;
use askama::Template;
use axum::Form;
use axum::Router;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::{get, post};
use serde::Deserialize;
use tokio::task;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
        .route("/register", get(self::get::register))
        .route("/register", post(self::post::register))
}

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate {
    next: Option<String>,
    token: String,
}
#[derive(Template)]
#[template(path = "auth/register.html")]
struct RegisterTemplate {
    token: String,
}

pub type AuthSession = axum_login::AuthSession<Backend>;

mod post {
    use super::*;
    use axum_csrf::CsrfToken;

    #[axum::debug_handler]
    pub async fn login(
        mut auth_session: AuthSession,
        token: CsrfToken,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        if token.verify(&creds.token).is_err() {
            return (StatusCode::UNAUTHORIZED).into_response();
        }
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
            Err(e) => {
                tracing::error!("{}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        if let Some(next) = creds.next {
            return Redirect::to(&next).into_response();
        };
        Redirect::to("/").into_response()
    }
    pub async fn register(
        State(state): State<AppState>,
        token: CsrfToken,
        Form(form): Form<CreateUser>,
    ) -> Result<impl IntoResponse, StatusCode> {
        if token.verify(&form.token).is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        let db = state.db;
        if form.password != form.repeat_password {
            todo!()
        }
        let id = Uuid::new_v4();
        let password_hash = task::spawn_blocking(|| password_auth::generate_hash(form.password))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let _user = sqlx::query_as!(
            User,
            "INSERT INTO users (id, username, password) VALUES ($1, $2, $3) RETURNING id, username, password",
            id, form.username, password_hash
        )
        .fetch_one(&db)
        .await.map_err(|e| {tracing::error!("{}",e); StatusCode::INTERNAL_SERVER_ERROR})?;

        Ok(Redirect::to("/login"))
    }
}

#[derive(Debug, Deserialize)]
struct NextUrl {
    next: Option<String>,
}

mod get {
    use axum_csrf::CsrfToken;

    use super::*;

    pub async fn login(
        token: CsrfToken,
        Query(NextUrl { next }): Query<NextUrl>,
    ) -> impl IntoResponse {
        let template = LoginTemplate {
            next,
            token: token.authenticity_token().unwrap(),
        };
        (token, Html(template.render().unwrap())).into_response()
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        if auth_session.logout().await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        Redirect::to("/").into_response()
    }
    pub async fn register(token: CsrfToken) -> impl IntoResponse {
        let template = RegisterTemplate {
            token: token.authenticity_token().unwrap(),
        };

        (token, Html(template.render().unwrap())).into_response()
    }
}
