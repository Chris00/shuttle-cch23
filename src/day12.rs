use std::time::Instant;
use axum::{extract::{Json, Path, State}, http::StatusCode};

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
) -> Result<String, StatusCode> {
    
    Ok(format!("{:?}", ulids))
}

pub async fn uuid_weekday(
    Path(_weekday): Path<u8>,
    Json(ulids): Json<Vec<String>>
) -> String {

    format!("{:?}", ulids)
}
