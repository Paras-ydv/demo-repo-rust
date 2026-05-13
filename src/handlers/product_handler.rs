use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ProductInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct ProductOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn product_handler(
    Path(id): Path<String>,
    Json(input): Json<ProductInput>,
) -> Json<ProductOutput> {
    Json(ProductOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778711395990