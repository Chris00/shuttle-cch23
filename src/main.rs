use axum::{routing::get, Router, extract::Path, http::StatusCode};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/*nums", get(day1));

    Ok(router.into())
}

macro_rules! fail_if {
    ($cond: expr) => {
        if $cond {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
}

async fn day1(Path(nums): Path<String>)
              -> Result<String, StatusCode>
{
    // No neutral element for xor.
    fail_if!(nums.is_empty());
    let mut nums = nums.split('/').map_while(|n| n.parse::<u32>().ok());
    let x = nums.next();
    fail_if!(x.is_none());
    let mut x = x.unwrap();
    for n in nums {
        x ^= n
    }
    Ok(format!("{}", x * x * x))
}
