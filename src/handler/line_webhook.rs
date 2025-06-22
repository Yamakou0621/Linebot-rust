use axum::{body::Body, http::StatusCode, response::IntoResponse};
use http_body_util::BodyExt;

//post/webhookの非同期関数　requestBodyを受け取ってhttpレスポンスを返す
pub async fn handle_line_webhook(body: Body) -> impl IntoResponse {
    //requestBodyを全て受け取ってcollectedオブジェクトにまとめる
    let collected = body.collect().await.unwrap();
    //バイト列を取り出す
    let bytes = collected.to_bytes();

    //バイト列をUTF-8にエンコーディングして標準出力
    println!("Received body: {:?}", String::from_utf8_lossy(&bytes));
    //httpsコード200を返す
    StatusCode::OK
}
