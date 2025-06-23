use serde::Deserialize;

//最上位構造。evnetに複数のlineEventを格納
#[derive(Debug, Deserialize)]
pub struct LineWebhookBody {
    pub events: Vec<LineEvent>,
}

// LineEvent は、1件のLINEイベント（メッセージなど）を表す構造体。
// r#type: イベント種別（例：message、followなど
// source: イベントの送信元（LineSource構造体）
// reply_token: このイベントに対して返信するためのトークン
// message: メッセージ本体（あれば）。Option型なので、無いこともある
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
//repleyTokenのためcamelcaseに変換
pub struct LineEvent {
    pub source: LineSource,
    pub r#type: String,
    pub reply_token: String,
    pub message: Option<LineMessage>,
}

// メッセージ内容（本文）がある場合に格納する構造体。
// r#type は text, image などのタイプ
// text はテキスト本文そのもの（Option型）
#[derive(Debug, Deserialize)]
pub struct LineMessage {
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LineSource {
    pub user_id: Option<String>,
}
