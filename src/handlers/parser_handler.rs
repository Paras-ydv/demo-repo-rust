use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ParserInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct ParserOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn parser_handler(
    Path(id): Path<String>,
    Json(input): Json<ParserInput>,
) -> Json<ParserOutput> {
    Json(ParserOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778736003947