use axum::extract::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ExtReindeer {
    name: String,
    strength: i32,
    speed: f64,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: i32,
}

#[derive(Serialize)]
pub struct Winners {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn contest(Json(r): Json<Vec<ExtReindeer>>) -> Json<Winners> {
    let s = r.iter().max_by(|r1, r2| r1.speed.total_cmp(&r2.speed)).unwrap();
    let h = r.iter().max_by_key(|r| r.height).unwrap();
    let m = r.iter().max_by_key(|r| r.snow_magic_power).unwrap();
    let c = r.iter().max_by_key(|r| r.candies).unwrap();
    Winners {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            s.strength, s.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            h.name, h.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            m.name, m.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            c.name, c.favorite_food
        ),
    }
    .into()
}

#[derive(Deserialize, Debug)]
pub struct Reindeer {
    #[allow(dead_code)]
    name: String,
    strength: i32,
}

pub async fn strength(Json(reindeers): Json<Vec<Reindeer>>) -> String {
    let s: i32 = reindeers.iter().map(|r| r.strength).sum();
    format!("{}", s)
}
