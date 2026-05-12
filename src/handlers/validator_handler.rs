use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ValidatorRequest {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct ValidatorResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_validator(req: web::Json<ValidatorRequest>) -> Result<HttpResponse> {
    let response = ValidatorResponse {
        success: true,
        message: format!("Processed: {}", req.name),
    };
    Ok(HttpResponse::Ok().json(response))
}
// auto-commit: 1778586808711