use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WebhookInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct WebhookOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn webhook_handler(
    Path(id): Path<String>,
    Json(input): Json<WebhookInput>,
) -> Json<WebhookOutput> {
    Json(WebhookOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778711390039