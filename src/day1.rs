use axum::{
    extract::Path,
    http::StatusCode,
};

pub async fn app(Path(nums): Path<String>) -> Result<String, StatusCode> {
    let mut nums = nums.split('/').map_while(|n| n.parse::<i32>().ok());
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
