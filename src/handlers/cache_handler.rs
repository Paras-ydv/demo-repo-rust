use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CacheInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct CacheOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn cache_handler(
    Path(id): Path<String>,
    Json(input): Json<CacheInput>,
) -> Json<CacheOutput> {
    Json(CacheOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778586784948