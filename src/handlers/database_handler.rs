use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DatabaseInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct DatabaseOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn database_handler(
    Path(id): Path<String>,
    Json(input): Json<DatabaseInput>,
) -> Json<DatabaseOutput> {
    Json(DatabaseOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778586780869