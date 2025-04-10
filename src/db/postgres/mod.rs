use axum::{
  extract::State,
  http::StatusCode,
};

use crate::web::app::AppState;


// we can extract the connection pool with `State`
pub(crate) async fn using_connection_pool_extractor(
  State(app_state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
  sqlx::query_scalar("select 'hello world from pg'")
      .fetch_one(&app_state.db)
      .await
      .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
  (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
