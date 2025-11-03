use ntex::web::HttpResponse;
use serde_json;
use crate::views::{ApiResponse, ApiError};

#[cfg(feature = "openapi")]
use utoipa::OpenApi;

pub async fn create_task() -> Result<HttpResponse, ApiError> {
    let response = serde_json::json!({"message": "create_task endpoint"});
    Ok(HttpResponse::Created().json(&response))
}

pub async fn get_task() -> Result<HttpResponse, ApiError> {
    let response = serde_json::json!({"message": "get_task endpoint"});
    Ok(HttpResponse::Ok().json(&response))
}

pub async fn list_tasks() -> Result<HttpResponse, ApiError> {
    let response = serde_json::json!({"message": "list_tasks endpoint"});
    Ok(HttpResponse::Ok().json(&response))
}

pub async fn update_task() -> Result<HttpResponse, ApiError> {
    let response = serde_json::json!({"message": "update_task endpoint"});
    Ok(HttpResponse::Ok().json(&response))
}

pub async fn delete_task() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::NoContent().finish())
}