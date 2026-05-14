use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct QueueRequest {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct QueueResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_queue(req: web::Json<QueueRequest>) -> Result<HttpResponse> {
    let response = QueueResponse {
        success: true,
        message: format!("Processed: {}", req.name),
    };
    Ok(HttpResponse::Ok().json(response))
}
// auto-commit: 1778741363276