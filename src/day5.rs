use axum::extract::{Json, Query};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bounds {
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
    split: Option<usize>,
}

pub async fn app(
    q: Query<Bounds>,
    Json(names): Json<Vec<String>>,
) -> String {
    let empty = vec![];
    let names = if q.offset <= names.len() {
        if let Some(limit) = q.limit {
            if q.offset + limit > names.len() {
                &names[q.offset ..]
            } else {
                &names[q.offset .. q.offset + limit]
            }
        } else {
            &names[q.offset ..]
        }
    } else {
        &empty
    };
    if let Some(split) = q.split {
        let names: Vec<_> = names.chunks(split).collect();
        serde_json::to_string(&names).unwrap()
    } else {
        serde_json::to_string(names).unwrap()
    }
}
