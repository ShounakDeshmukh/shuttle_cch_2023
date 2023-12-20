use reqwest::StatusCode;


// Task -1
pub async fn hello_world() -> &'static str {
    "Hello, world!"
}
// Task -1 Extra
pub async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
