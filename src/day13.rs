use std::collections::HashMap;

use axum::{extract::State, Json, debug_handler};
use serde::Deserialize;

pub struct DB {
    entries: Vec<Gift>
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Gift {
    id: u32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

impl DB {
    pub fn new() -> Self {
        DB { entries: vec![] }
    }
}

pub async fn orders_total(
    State(state): State<super::AppState>
) -> String {
    let gifts = state.day13.lock().expect("Poisoned mutex");
    let sum: i32 = gifts.entries.iter().map(|g| g.quantity).sum();
    format!("{{\"total\": {sum}}}")
}

pub async fn popular(State(state): State<super::AppState>) -> String {
    let gifts = state.day13.lock().expect("Poisoned mutex");
    let mut pq = HashMap::new();
    for g in &gifts.entries {
        pq.entry(&g.gift_name)
            .and_modify(|n| *n += g.quantity)
            .or_insert(g.quantity);
    }
    let mut popular = ""; // Most popular
    let mut n1 = 0;
    let mut n2 = 0;
    for (g, &n) in &pq {
        if n >= n1 {
            n2 = n1;
            n1 = n;
            popular = g;
        } else if n >= n2 {
            n2 = n;
        }
    }
    if n1 > n2 {
        format!("{{\"popular\": \"{}\"}}", popular)
    } else {
        "{\"popular\": null}".to_string()
    }
}

#[debug_handler]
pub async fn orders(
    State(state): State<super::AppState>,
    Json(mut orders): Json<Vec<Gift>>,
) {
    let gifts = &mut state.day13.lock().expect("Poisoned mutex");
    gifts.entries.append(&mut orders)
}

pub async fn reset(
    State(state): State<super::AppState>
) {
    state.day13.lock().unwrap()
        .entries.clear()
}

pub async fn sql() -> &'static str {
    "20231213"
}
