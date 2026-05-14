use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FormatterInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct FormatterOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn formatter_handler(
    Path(id): Path<String>,
    Json(input): Json<FormatterInput>,
) -> Json<FormatterOutput> {
    Json(FormatterOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778737108790