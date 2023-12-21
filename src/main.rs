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
mod day5;
mod day6;
mod day7;
mod day8;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day18;
mod day19;
mod day20;

#[derive(Clone, Debug)]
struct AppState {
    day12: Arc<Mutex<HashMap<String, Instant>>>,
    day13: Arc<Mutex<day13::DB>>,
    day18: Arc<Mutex<day18::Regions>>,
    day19: Arc<Mutex<day19::Views>>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            day12: Arc::new(Mutex::new(HashMap::new())),
            day13: Arc::new(Mutex::new(day13::DB::new())),
            day18: Arc::new(Mutex::new(day18::empty_regions())),
            day19: Arc::new(Mutex::new(day19::Views::new())),
        }
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let appstate = AppState::new();
    let router = Router::new()
        .route("/", get(day_1::it_works))
        .route("/-1/error", get(day_1::error))
        .route("/1/*nums", get(day1::app))
        .route("/4/strength", post(day4::strength))
        .route("/4/contest", post(day4::contest))
        .route("/5", post(day5::app))
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
        .route("/13/sql", get(day13::sql))
        .route("/13/reset", post(day13::reset))
        .route("/13/orders", post(day13::orders))
        .route("/13/orders/total", get(day13::orders_total))
        .route("/13/orders/popular", get(day13::popular))
        .route("/14/unsafe", post(day14::render_html_unsafe))
        .route("/14/safe", post(day14::render_html))
        .route("/15/nice", post(day15::nice))
        .route("/15/game", post(day15::game))
        .route("/18/reset", post(day18::reset))
        .route("/18/orders", post(day18::orders))
        .route("/18/regions", post(day18::regions))
        .route("/18/regions/total", get(day18::total))
        .route("/18/regions/top_list/:number", get(day18::top_list))
        .route("/19/ws/ping", get(day19::ping))
        .route("/19/reset", post(day19::reset))
        .route("/19/views", get(day19::views))
        .route("/19/ws/room/:number/user/:string", get(day19::room))
        .route("/20/archive_files", post(day20::archive_files))
        .route("/20/archive_files_size", post(day20::archive_files_size))
        .route("/20/cookie", post(day20::cookie))
        .with_state(appstate);

    Ok(router.into())
}
