use crate::handler::line_webhook::handle_line_webhook;
use axum::{Router, routing::post};

pub fn create_router() -> Router {
    Router::new().route("/webhook", post(handle_line_webhook))
}
