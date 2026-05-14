use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaymentInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn payment_handler(
    Path(id): Path<String>,
    Json(input): Json<PaymentInput>,
) -> Json<PaymentOutput> {
    Json(PaymentOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778741426624