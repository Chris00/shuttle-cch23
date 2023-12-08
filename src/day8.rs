use axum::{extract::Path, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Pokemon {
    weight: u64,
}

async fn get_pokemon(id: u64) -> eyre::Result<Pokemon> {
    Ok(reqwest::get(&format!("https://pokeapi.co/api/v2/pokemon/{id}"))
        .await?
        .json::<Pokemon>()
        .await?)
}

async fn compute_momentum(id: u64) -> eyre::Result<f64> {
    const G: f64 = 9.825; // m/s²
    let p = get_pokemon(id).await?;
    let m = p.weight as f64 / 10.; // kg
    // Fall (with initial 0 speed) from 10 m
    let v = (20. * G).sqrt();
    Ok(m * v)
}

pub async fn drop(Path(id): Path<u64>) -> Result<String, StatusCode> {
    let drop = compute_momentum(id).await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(format!("{}", drop))
}

pub async fn weight(Path(id): Path<u64>) -> Result<String, StatusCode> {
    let p = get_pokemon(id).await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    // Weight is in hectograms (https://pokeapi.co/docs/v2#pokemon)
    // but must be returned in kg.  Note that not all weights are
    // multuple of 10 (https://pokeapi.co/api/v2/pokemon/1).
    Ok(format!("{}", p.weight as f64 / 10.))
}
