use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FormatterRequest {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct FormatterResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_formatter(req: web::Json<FormatterRequest>) -> Result<HttpResponse> {
    let response = FormatterResponse {
        success: true,
        message: format!("Processed: {}", req.name),
    };
    Ok(HttpResponse::Ok().json(response))
}
// auto-commit: 1778711407306