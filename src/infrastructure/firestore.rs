use gcloud_sdk::GoogleApi;
use gcloud_sdk::google::firestore::v1::{
    Document, GetDocumentRequest, UpdateDocumentRequest, Value,
    firestore_client::FirestoreClient, value::ValueType,
};
use std::collections::HashMap;
use tonic::Request;

pub async fn add_amount(
    project_id: &str,
    user_id: &str,
    amount: u32,
) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
    // 1. FirestoreClient の初期化
    let cloud_prefix = format!("projects/{}/databases/(default)", project_id);
    let firestore_client: GoogleApi<FirestoreClient<_>> =
        GoogleApi::from_function(
            FirestoreClient::new,
            "https://firestore.googleapis.com",
            Some(cloud_prefix.clone()),
        )
        .await?;

    // ドキュメントのパス
    let doc_name = format!("{}/documents/users/{}", cloud_prefix, user_id);

    // 2. ドキュメント取得
    let current_total = match firestore_client
        .get()
        .get_document(Request::new(GetDocumentRequest {
            name: doc_name.clone(),
            ..Default::default()
        }))
        .await
    {
        Ok(resp) => {
            let fields = resp.into_inner().fields;
            match fields.get("total") {
                Some(Value {
                    value_type: Some(ValueType::IntegerValue(v)),
                }) => *v as u32,
                _ => 0,
            }
        }
        Err(_) => 0,
    };

    let new_total = current_total + amount;

    // 3. ドキュメント更新
    let mut fields = HashMap::new();
    fields.insert(
        "total".to_string(),
        Value { value_type: Some(ValueType::IntegerValue(new_total as i64)) },
    );

    let document =
        Document { name: doc_name.clone(), fields, ..Default::default() };

    firestore_client
        .get()
        .update_document(Request::new(UpdateDocumentRequest {
            document: Some(document),
            update_mask: None,
            ..Default::default()
        }))
        .await?;

    Ok(new_total)
}
