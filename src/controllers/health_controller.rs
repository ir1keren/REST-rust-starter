use ntex::web::HttpResponse;
use chrono::Utc;

#[cfg(feature = "openapi")]
use utoipa::ToResponse;

#[derive(serde::Serialize)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

pub async fn health_check() -> HttpResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    
    HttpResponse::Ok().json(&response)
}