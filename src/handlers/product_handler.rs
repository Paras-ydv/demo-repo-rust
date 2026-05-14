use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ProductRequest {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_product(req: web::Json<ProductRequest>) -> Result<HttpResponse> {
    let response = ProductResponse {
        success: true,
        message: format!("Processed: {}", req.name),
    };
    Ok(HttpResponse::Ok().json(response))
}
// auto-commit: 1778737097606