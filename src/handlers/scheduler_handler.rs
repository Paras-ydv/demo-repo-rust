use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SchedulerInput {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct SchedulerOutput {
    pub result: String,
    pub timestamp: i64,
}

pub async fn scheduler_handler(
    Path(id): Path<String>,
    Json(input): Json<SchedulerInput>,
) -> Json<SchedulerOutput> {
    Json(SchedulerOutput {
        result: format!("Processed {} with {}", id, input.data),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
// auto-commit: 1778455024829