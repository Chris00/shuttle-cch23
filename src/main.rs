use axum::{routing::get, Router, extract::Path};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/:num1/:num2", get(day1));

    Ok(router.into())
}

async fn day1(Path((num1, num2)): Path<(u32, u32)>) -> String {
    let x = num1 ^ num2;
    format!("{}", x * x * x)
}
