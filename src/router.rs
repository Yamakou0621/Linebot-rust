use crate::handler::line_webhook::handle_line_webhook;
use axum::{Router, routing::post};

//Router型を返す
pub fn create_router() -> Router {
    //Routerを作成してpostメソッドを返す。
    Router::new().route("/webhook", post(handle_line_webhook))
}
