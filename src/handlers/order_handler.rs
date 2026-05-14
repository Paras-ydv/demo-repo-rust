use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OrderRequest {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_order(req: web::Json<OrderRequest>) -> Result<HttpResponse> {
    let response = OrderResponse {
        success: true,
        message: format!("Processed: {}", req.name),
    };
    Ok(HttpResponse::Ok().json(response))
}
// auto-commit: 1778737587148