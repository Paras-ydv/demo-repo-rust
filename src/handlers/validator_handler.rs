use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ValidatorInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct ValidatorOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn validator_handler(
    Path(id): Path<String>,
    Json(input): Json<ValidatorInput>,
) -> Json<ValidatorOutput> {
    Json(ValidatorOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778455436562