use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct StorageInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct StorageOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn storage_handler(
    Path(id): Path<String>,
    Json(input): Json<StorageInput>,
) -> Json<StorageOutput> {
    Json(StorageOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778711419100