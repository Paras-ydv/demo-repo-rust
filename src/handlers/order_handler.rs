use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OrderInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct OrderOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn order_handler(
    Path(id): Path<String>,
    Json(input): Json<OrderInput>,
) -> Json<OrderOutput> {
    Json(OrderOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778741407122