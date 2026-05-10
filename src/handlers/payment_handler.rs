use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaymentRequest {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_payment(req: web::Json<PaymentRequest>) -> Result<HttpResponse> {
    let response = PaymentResponse {
        success: true,
        message: format!("Processed: {}", req.name),
    };
    Ok(HttpResponse::Ok().json(response))
}
// auto-commit: 1778455021614