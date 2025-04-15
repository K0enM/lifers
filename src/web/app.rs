use axum::{
    extract::Request,
    http::{self, HeaderValue},
    middleware::{self, Next},
    response::Response,
};
use axum_csrf::{CsrfConfig, CsrfLayer};
use axum_login::{AuthManagerLayerBuilder, login_required};
use axum_messages::MessagesManagerLayer;
use sqlx::{PgPool, postgres::PgPoolOptions};
use time::Duration;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::{ExpiredDeletion, SessionManagerLayer, cookie::Key};
use tower_sessions_sqlx_store::PostgresStore;

use crate::{
    web::entities::users::Backend,
    web::{
        auth, fallback,
        pages::{dashboard, index},
    },
};

pub struct App {
    pub state: AppState,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
}

impl App {
    pub async fn new() -> Result<App, Box<dyn std::error::Error>> {
        let db_connection_str = std::env::var("DATABASE_URL").expect("cant find database url");

        tracing::info!("connecting to database: {}", db_connection_str);

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("can't connect to database");

        sqlx::migrate!().run(&pool).await?;

        let state = AppState { db: pool };

        Ok(Self { state })
    }

    pub async fn serve(self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let session_store = PostgresStore::new(self.state.db.clone());
        session_store.migrate().await?;

        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        let session_key_str = std::env::var("SESSION_KEY")?;
        let key = Key::from(&hex::decode(session_key_str)?);

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::days(1)))
            .with_signed(key);

        let csrf_config = CsrfConfig::default();

        let backend = Backend::new(self.state.db.clone());
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let app = dashboard::router()
            .route_layer(login_required!(Backend, login_url = "/login"))
            .merge(index::router())
            .merge(auth::router())
            .nest_service(
                "/static",
                ServiceBuilder::new()
                    .layer(middleware::from_fn(set_no_cache_control))
                    .service(ServeDir::new("static")),
            )
            .with_state(self.state)
            .layer(MessagesManagerLayer)
            .layer(CsrfLayer::new(csrf_config))
            .layer(auth_layer)
            .fallback(fallback::handler_404);

        let addr_str = format!("[::]:{}", port);
        tracing::info!("listening on {}", addr_str);
        let listener = TcpListener::bind(addr_str.as_str())
            .await
            .expect("failed to bind");
        axum::serve(
            listener,
            app.layer(TraceLayer::new_for_http()).into_make_service(),
        )
        .with_graceful_shutdown(shutdown_signal())
        .await?;

        deletion_task.await??;

        Ok(())
    }
}
// graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}

async fn set_no_cache_control(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        http::header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache"),
    );
    response
}
