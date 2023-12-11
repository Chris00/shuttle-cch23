use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
mod day_1;
mod day1;
mod day4;
mod day6;
mod day7;
mod day8;
mod day11;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_1::it_works))
        .route("/-1/error", get(day_1::error))
        .route("/1/*nums", get(day1::app))
        .route("/4/strength", post(day4::strength))
        .route("/4/contest", post(day4::contest))
        .route("/6" , post(day6::app))
        .route("/7/decode", get(day7::decode))
        .route("/7/bake", get(day7::bake))
        .route("/8/weight/:pokedex_number", get(day8::weight))
        .route("/8/drop/:pokedex_number", get(day8::drop))
        .nest_service("/11/assets", ServeDir::new("assets"))
        .route("/11/red_pixels", post(day11::red_pixels));

    Ok(router.into())
}
