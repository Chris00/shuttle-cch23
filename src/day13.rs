use axum::{extract::State, http::StatusCode};
use sqlx::Row;

pub async fn sql(
    State(state): State<super::AppState>
) -> Result<String, (StatusCode, String)> {
    match sqlx::query("SELECT 20231213")
        .fetch_one(&state.pool)
        .await
    {
        Ok(r) => Ok(format!("{:?}", r.get::<&str,_>(1))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
