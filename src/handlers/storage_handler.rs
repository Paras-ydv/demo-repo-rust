use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct StorageRequest {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct StorageResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_storage(req: web::Json<StorageRequest>) -> Result<HttpResponse> {
    let response = StorageResponse {
        success: true,
        message: format!("Processed: {}", req.name),
    };
    Ok(HttpResponse::Ok().json(response))
}
// auto-commit: 1778455917919