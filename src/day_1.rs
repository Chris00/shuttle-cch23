use axum::http::StatusCode;

pub async fn it_works() -> &'static str {
    "It works"
}

pub async fn error() -> Result<(), StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
