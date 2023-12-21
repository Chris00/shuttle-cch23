use std::time::Instant;
use axum::{extract::{Json, Path, State}, http::StatusCode};
use ulid::Ulid;
use uuid::Uuid;
use chrono::prelude::*;

pub async fn save(Path(key): Path<String>,
                  State(state): State<super::AppState>)
{
    let now = Instant::now();
    let mut m = state.day12.lock().expect("Mutex poisoned");
    m.insert(key, now);
}

pub async fn load(Path(key): Path<String>,
                  State(state): State<super::AppState>
) -> Result<String, StatusCode> {
    let m = state.day12.lock().expect("Mutex poisoned");
    if let Some(t) = m.get(&key) {
        Ok(format!("{}", t.elapsed().as_secs()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn uuid(
    Json(ulids): Json<Vec<String>>
) -> Result<Json<Vec<String>>, StatusCode> {
    let ulids: Vec<_> = ulids.iter()
        .filter_map(|u| Ulid::from_string(u).ok())
        .map(|u| format!("{}", Uuid::from_u128(u.0)))
        .rev()
        .collect();
    Ok(Json(ulids))
}

pub async fn uuid_weekday(
    Path(weekday): Path<u8>,
    Json(ulids): Json<Vec<String>>
) -> String {
    let ulids = ulids.iter()
        .filter_map(|u| Ulid::from_string(u).ok());
    let mut n_christmas = 0;
    let mut n_weekday = 0;
    let mut n_future = 0;
    let mut n_lsb = 0;
    let now = Utc::now();
    for u in ulids {
        let t: DateTime<Utc> = u.datetime().into();
        if t.month() == 12 && t.day() == 24 {
            n_christmas += 1;
        }
        if t.weekday() as u8 == weekday {
            n_weekday += 1;
        }
        if t > now {
            n_future += 1;
        }
        if u.0 & 1 == 1 {
            n_lsb += 1;
        }
    }
    format!("{{\"christmas eve\": {n_christmas}, \
            \"weekday\": {n_weekday}, \
            \"in the future\": {n_future}, \
            \"LSB is 1\": {n_lsb}}}")
}
