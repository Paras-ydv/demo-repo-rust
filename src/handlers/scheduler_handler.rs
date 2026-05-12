use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SchedulerRequest {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct SchedulerResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_scheduler(req: web::Json<SchedulerRequest>) -> Result<HttpResponse> {
    let response = SchedulerResponse {
        success: true,
        message: format!("Processed: {}", req.name),
    };
    Ok(HttpResponse::Ok().json(response))
}
// auto-commit: 1778586772905