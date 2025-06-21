use axum::{body::Body, http::StatusCode, response::IntoResponse};
use http_body_util::BodyExt;

pub async fn handle_line_webhook(body: Body) -> impl IntoResponse {
    let collected = body.collect().await.unwrap();
    let bytes = collected.to_bytes();

    println!("Received body: {:?}", String::from_utf8_lossy(&bytes));
    StatusCode::OK
}
