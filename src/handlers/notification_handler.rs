use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NotificationInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn notification_handler(
    Path(id): Path<String>,
    Json(input): Json<NotificationInput>,
) -> Json<NotificationOutput> {
    Json(NotificationOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778711397965