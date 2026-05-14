use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NotificationRequest {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_notification(req: web::Json<NotificationRequest>) -> Result<HttpResponse> {
    let response = NotificationResponse {
        success: true,
        message: format!("Processed: {}", req.name),
    };
    Ok(HttpResponse::Ok().json(response))
}
// auto-commit: 1778732136150