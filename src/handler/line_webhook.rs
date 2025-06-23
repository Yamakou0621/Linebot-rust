use crate::domain::consumption::Consumption;
use crate::infrastructure::firestore::add_amount;
use crate::infrastructure::line::reply_message;
use crate::model::line_event::LineWebhookBody;
use axum::{body::Body, http::StatusCode, response::IntoResponse};
use axum_macros::debug_handler;
use http_body_util::BodyExt;
use hyper::Request;
use unicode_normalization::UnicodeNormalization;

//post/webhookの非同期関数　requestBodyを受け取ってhttpレスポンスを返す
#[debug_handler]
pub async fn handle_line_webhook(req: Request<Body>) -> impl IntoResponse {
    //requestBodyを全て受け取ってcollectedオブジェクトにまとめる
    let collected = req.collect().await.unwrap();
    //バイト列を取り出す
    let bytes = collected.to_bytes();
    let body_str = String::from_utf8_lossy(&bytes);
    println!("Received body: {}", body_str);

    //json構造体に変換
    if let Ok(parsed) = serde_json::from_str::<LineWebhookBody>(&body_str) {
        for event in parsed.events {
            if event.r#type == "message" {
                if let Some(msg) = event.message {
                    if let Some(text) = msg.text {
                        println!("User said: {}", text);

                        //項目 金額 単価　頻度　の形式でパース
                        let text = text.trim().nfc().collect::<String>();
                        let parts: Vec<&str> =
                            text.trim().split_whitespace().collect();

                        if parts.len() != 4 {
                            reply_message(
                            &event.reply_token,
                                "形式エラー：半角スペースで「項目 金額 購入間隔日数 1回あたりの購入個数」の4つを入力してください（例: タバコ 500 1 1）",
                            )
                            .await;
                            return StatusCode::OK;
                        }

                        let item = parts[0].to_string();
                        let price_str = parts[1];
                        let days_str = parts[2];
                        let frequency_str = parts[3];

                        // 数値変換
                        let price = price_str.parse::<u32>();
                        let days = days_str.parse::<u32>();
                        let frequency = frequency_str.parse::<u32>();

                        if price.is_err() {
                            reply_message(
                                &event.reply_token,
                                &format!("金額が不正です：「{}」は数字として認識できません。半角数字で入力してください。", price_str),
                            )
                            .await;
                        } else if days.is_err() {
                            reply_message(
                                &event.reply_token,
                                &format!("日数が不正です：「{}」は数字として認識できません。半角数字で入力してください。", days_str),
                            )
                            .await;
                        } else if frequency.is_err() {
                            reply_message(
                                &event.reply_token,
                                &format!("頻度が不正です：「{}」は数字として認識できません。半角数字で入力してください。", frequency_str),
                            )
                            .await;
                        } else {
                            let consumption = Consumption {
                                item: item.clone(),
                                unit_price: price.unwrap(),
                                period_days: days.unwrap(),
                                frequency_per_period: frequency.unwrap(),
                                total_days: 365,
                            };
                            let total = consumption.yearly_total();

                            if let Some(user_id) =
                                event.source.user_id.as_deref()
                            {
                                let project_id =
                                    std::env::var("GOOGLE_CLOUD_PROJECT")
                                        .expect("環境変数が設定されていません");

                                match add_amount(&project_id, user_id, total)
                                    .await
                                {
                                    Ok(_) => {
                                        let message = format!(
                                            "{}を{}日に{}回買うと年間約{}円使ってますね！",
                                            item,
                                            days_str,
                                            frequency_str,
                                            total
                                        );

                                        reply_message(
                                            &event.reply_token,
                                            &message,
                                        )
                                        .await;
                                    }
                                    Err(e) => {
                                        eprintln!("Firestore error: {}", e);
                                        reply_message(
                                            &event.reply_token,
                                            "サーバーエラー：累計保存に失敗しました",
                                        )
                                        .await;
                                    }
                                }
                            } else {
                                reply_message(
                                    &event.reply_token,
                                    "ユーザーIDが取得できませんでした",
                                )
                                .await;
                            }
                        }
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
