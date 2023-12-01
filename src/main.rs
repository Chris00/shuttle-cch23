use axum::{routing::get, Router, extract::Path, http::StatusCode};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/*nums", get(day1));

    Ok(router.into())
}

async fn day1(Path(nums): Path<String>)
              -> Result<String, StatusCode>
{
    let mut nums = nums.split('/').map_while(|n| n.parse::<u32>().ok());
    // No neutral element for xor.
    let x = nums.next();
    if x.is_none() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let mut x = x.unwrap();
    for n in nums {
        x ^= n
    }
    Ok(format!("{}", x * x * x))
}
