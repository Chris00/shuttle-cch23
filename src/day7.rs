use std::str;
use axum::{http::StatusCode, Json};
use axum_extra::extract::CookieJar;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};

type DynErr = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize, Debug)]
struct Quantities {
    flour: u32,
    sugar: u32,
    butter: u32,
    #[serde(rename = "baking powder")]
    baking_powder: u32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u32,
}

#[derive(Deserialize, Debug)]
struct Baking {
    recipe: Quantities,
    pantry: Quantities,
}

#[derive(Serialize, Debug)]
pub struct Cookies {
  cookies: u32,
  pantry: Quantities,
}

fn compute_bake(cookie: CookieJar) -> Result<Cookies, DynErr> {
    let (recipe, n) = decode_recipe(cookie)?;
    let Json(baking): Json<Baking> = Json::from_bytes(&recipe[..n])?;
    let recipe = baking.recipe;
    let mut pantry = baking.pantry;
    let cookies =
        (pantry.flour / recipe.flour)
            .min(pantry.sugar / recipe.sugar)
            .min(pantry.butter / recipe.butter)
            .min(pantry.baking_powder / recipe.baking_powder)
            .min(pantry.chocolate_chips / recipe.chocolate_chips);
    pantry.flour -= cookies * recipe.flour;
    pantry.sugar -= cookies * recipe.sugar;
    pantry.butter -= cookies * recipe.butter;
    pantry.baking_powder -= cookies * recipe.baking_powder;
    pantry.chocolate_chips -= cookies * recipe.chocolate_chips;
    Ok(Cookies { cookies, pantry })
}

pub async fn bake(cookie: CookieJar) -> Result<Json<Cookies>, StatusCode> {
    let c = compute_bake(cookie)
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(c))
}

pub async fn decode(cookie: CookieJar) -> Result<String, StatusCode> {
    let (recipe, n) = decode_recipe(cookie)
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(str::from_utf8(&recipe[..n])
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?.to_string())
}

fn decode_recipe(cookie: CookieJar) -> Result<([u8; 256], usize), DynErr> {
    let cookie = cookie.get("recipe")
        .ok_or("Cookie 'recipe' not present")?
        .value();
    let mut recipe = [0; 256];
    let n = general_purpose::STANDARD.decode_slice(cookie, &mut recipe)?;
    Ok((recipe, n))
}
