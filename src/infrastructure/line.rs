use crate::config::get_env;
use reqwest::Client;
use serde_json::json;

pub async fn reply_message(reply_token: &str, message: &str) {
    //requestのhttpクライアントを初期化
    let client = Client::new();

    let token = get_env("LINE_CHANNEL_TOKEN");

    //postするjson
    let body = json!({
        "replyToken": reply_token,
        "messages": [
            {
                "type": "text",
                "text": message
            }
        ]
    });

    //lineの返信apiにpostリクエストを送る
    let res = client
        .post("https://api.line.me/v2/bot/message/reply")
        //アクセストークンを認証トークンとしてヘッダに付与
        .bearer_auth(token)
        .json(&body)
        .send()
        .await;

    match res {
        Ok(r) => println!("Replied: {:?}", r.status()),
        Err(e) => eprintln!("Failed to reply: {}", e),
    }
}
