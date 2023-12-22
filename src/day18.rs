use std::collections::HashMap;

use axum::{extract::{Path, State}, Json, debug_handler};
use serde::{Deserialize, Serialize};

type Gift = crate::day13::Gift;

#[derive(Deserialize, Debug)]
pub struct Region {
    id: i32,
    name: String,
}

pub type Regions = Vec<Region>;

pub fn empty_regions() -> Regions { vec![] }

#[derive(Serialize, Debug)]
pub struct RegionTotal {
    region: String,
    total: i32,
}

#[debug_handler]
pub async fn total(
    State(state): State<super::AppState>
) -> Json<Vec<RegionTotal>> {
    let orders = &state.day13.lock().unwrap().entries;
    let regions = &state.day18.lock().unwrap();
    let mut totals = vec![];
    for r in regions.iter() {
        let o = orders.iter().filter_map(|g| {
            if g.region_id == r.id { Some(g.quantity) } else { None }});
        // There may be some orders (with negative quantities and the
        // total quantity being 0 !!!)
        if o.clone().count() > 0 {
            totals.push(RegionTotal {
                region: r.name.clone(),
                total: o.sum()
            })
        }
    }
    totals.sort_by(|t1, t2| t1.region.cmp(&t2.region));
    Json(totals)
}

#[derive(Serialize, Debug)]
pub struct RegionGifts {
    region: String,
    top_gifts: Vec<String>,
}

pub async fn top_list(
    Path(number): Path<i32>,
    State(state): State<super::AppState>,
) -> Json<Vec<RegionGifts>> {
    let orders = &state.day13.lock().unwrap().entries;
    let regions = &state.day18.lock().unwrap();
    let mut top = vec![];
    for r in regions.iter() {
        let mut gifts = HashMap::new();
        for g in orders {
            if g.region_id == r.id {
                gifts.entry(g.gift_name.clone())
                    .and_modify(|n| *n += g.quantity)
                    .or_insert(g.quantity);
            }
        }
        let mut gifts: Vec<_> = gifts.drain().collect();
        gifts.sort_by(|(name1, n1), (name2, n2)| {
            n1.cmp(n2).reverse().then(name1.cmp(name2))
        });
        let n = number.max(0) as usize;
        let top_gifts = gifts.into_iter().take(n)
            .map(|(name, _)| name).collect();
        top.push(RegionGifts { region: r.name.clone(), top_gifts })
    }
    top.sort_by(|r1, r2| r1.region.cmp(&r2.region));
    Json(top)
}


pub async fn regions(
    State(state): State<super::AppState>,
    Json(mut regions): Json<Regions>,
) {
    let r = &mut state.day18.lock().unwrap();
    r.append(&mut regions)
}

pub async fn orders(state: State<super::AppState>,
                    orders: Json<Vec<Gift>>)
{
    crate::day13::orders(state, orders).await
}

pub async fn reset(State(state): State<super::AppState>) {
    state.day18.lock().unwrap().clear();
    crate::day13::reset(State(state)).await;
}
