use std::str;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use base64::{Engine as _, engine::general_purpose};
use serde_json::Value;
use eyre::eyre;

fn compute_bake(cookie: CookieJar) -> eyre::Result<String> {
    let recipe = decode_recipe(cookie)?;
    let baking: Value = serde_json::from_str(&recipe)?;
    let recipe;
    if let Value::Object(r) = &baking["recipe"] {
        recipe = r
    } else {
        return Err(eyre!("No 'recipe' field"))
    }
    let mut pantry;
    if let Value::Object(p) = &baking["pantry"] {
        pantry = p.clone()
    } else {
        return Err(eyre!("No 'pantry' field"))
    }
    let mut cookies = u64::MAX;
    for (i, c) in recipe.iter() {
        match (c, pantry.get(i)) {
            (Value::Number(c), Some(Value::Number(avail))) => {
                if let (Some(c), Some(avail)) = (c.as_u64(), avail.as_u64()) {
                    cookies = cookies.min(avail / c)
                } else {
                    cookies = 0;
                    break
                }
            }
            _ => {
                // Ingredient not available or quantity not a number
                cookies = 0;
                break
            }
        }
    }
    if cookies > 0 {
        // Subtract the ingredients used.
        for (i, avail) in pantry.iter_mut() {
            match (avail, recipe.get(i)) {
                (Value::Number(avail), Some(Value::Number(c))) => {
                    if let (Some(a), Some(c)) = (avail.as_u64(), c.as_u64()) {
                        *avail = (a - cookies * c).into()
                    }
                }
                _ => (),
            }
        }
    }
    Ok(format!(r#"{{ "cookies": {cookies}, "pantry": {} }}"#,
        serde_json::to_string(&pantry)?))
}

pub async fn bake(cookie: CookieJar) -> Result<String, StatusCode> {
    compute_bake(cookie)
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn decode(cookie: CookieJar) -> Result<String, StatusCode> {
    Ok(decode_recipe(cookie)
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?)
}

fn decode_recipe(cookie: CookieJar) -> eyre::Result<String> {
    let cookie = cookie.get("recipe")
        .ok_or(eyre!("Cookie 'recipe' not present"))?
        .value();
    let mut recipe = [0; 256];
    let n = general_purpose::STANDARD.decode_slice(cookie, &mut recipe)?;
    Ok(str::from_utf8(&recipe[..n])?.to_string())
}