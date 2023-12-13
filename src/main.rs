use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};
mod day_1;
mod day1;
mod day4;
mod day6;
mod day7;
mod day8;
mod day11;
mod day12;

#[derive(Clone)]
struct AppState {
    day12: Arc<Mutex<HashMap<String, Instant>>>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState { day12: Arc::new(Mutex::new(HashMap::new())) }
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let appstate = AppState::default();
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
        .route("/11/red_pixels", post(day11::red_pixels))
        .route("/12/save/:string", post(day12::save))
        .route("/12/load/:string", get(day12::load))
        .route("/12/ulids", post(day12::uuid))
        .route("/12/ulids/:weekday", post(day12::uuid_weekday))
        .with_state(appstate);

    Ok(router.into())
}
