use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ConfigRequest {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_config(req: web::Json<ConfigRequest>) -> Result<HttpResponse> {
    let response = ConfigResponse {
        success: true,
        message: format!("Processed: {}", req.name),
    };
    Ok(HttpResponse::Ok().json(response))
}
// auto-commit: 1778711417250