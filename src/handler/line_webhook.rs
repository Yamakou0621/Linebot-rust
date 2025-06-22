use crate::infrastructure::line::reply_message;
use crate::model::line_event::LineWebhookBody;
use axum::{body::Body, http::StatusCode, response::IntoResponse};
use http_body_util::BodyExt;

//post/webhookの非同期関数　requestBodyを受け取ってhttpレスポンスを返す
pub async fn handle_line_webhook(body: Body) -> impl IntoResponse {
    //requestBodyを全て受け取ってcollectedオブジェクトにまとめる
    let collected = body.collect().await.unwrap();
    //バイト列を取り出す
    let bytes = collected.to_bytes();

    let body_str = String::from_utf8_lossy(&bytes);
    println!("Received body: {}", body_str);

    if let Ok(parsed) = serde_json::from_str::<LineWebhookBody>(&body_str) {
        for event in parsed.events {
            if event.r#type == "message" {
                if let Some(msg) = event.message {
                    if let Some(text) = msg.text {
                        println!("User said: {}", text);
                        reply_message(&event.reply_token, "テスト応答だよ！")
                            .await;
                    }
                }
            }
        }
    } else {
        eprintln!("Parse error");
    }

    //バイト列をUTF-8にエンコーディングして標準出力
    println!("Received body: {:?}", String::from_utf8_lossy(&bytes));
    //httpsコード200を返す
    StatusCode::OK
}
