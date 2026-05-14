use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ApiInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct ApiOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn api_handler(
    Path(id): Path<String>,
    Json(input): Json<ApiInput>,
) -> Json<ApiOutput> {
    Json(ApiOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778736000058