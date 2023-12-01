use axum::{
    extract:: Path,
    http::StatusCode,
    routing::get,
    Router,
};

// Task -1
async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn cube_the_bits(Path(nums): Path<String>) -> String {
    let numbers = nums
        .split("/")
        .map(|n| n.parse::<u64>().unwrap())
        .fold(0, |acc, n| acc ^ n)
        .pow(3)
        .to_string();

    numbers
}
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/*nums", get(cube_the_bits));
    Ok(router.into())
}
