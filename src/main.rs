use axum::{
    routing::{get, post},
    Router,
};
mod day1;
mod day4;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/*nums", get(day1::app))
        .route("/4/strength", post(day4::strength))
        .route("/4/contest", post(day4::contest));

    Ok(router.into())
}
