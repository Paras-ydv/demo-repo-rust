use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct AuthOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn auth_handler(
    Path(id): Path<String>,
    Json(input): Json<AuthInput>,
) -> Json<AuthOutput> {
    Json(AuthOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778741394522