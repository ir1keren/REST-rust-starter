use crate::views::ApiError;
use ntex::web::HttpResponse;
use serde_json;

#[cfg(feature = "openapi")]
use utoipa;

#[cfg_attr(feature = "openapi", utoipa::path(
    post,
    path = "/api/v1/tasks",
    tag = "tasks",
    request_body = TaskCreate,
    responses(
        (status = 201, description = "Task created successfully", body = ApiResponse<Task>),
        (status = 400, description = "Invalid request", body = ApiResponse<()>)
    )
))]
pub async fn create_task() -> Result<HttpResponse, ApiError> {
    let response = serde_json::json!({"message": "create_task endpoint"});
    Ok(HttpResponse::Created().json(&response))
}

#[cfg_attr(feature = "openapi", utoipa::path(
    get,
    path = "/api/v1/tasks/{id}",
    tag = "tasks",
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task found", body = ApiResponse<Task>),
        (status = 404, description = "Task not found", body = ApiResponse<()>)
    )
))]
pub async fn get_task() -> Result<HttpResponse, ApiError> {
    let response = serde_json::json!({"message": "get_task endpoint"});
    Ok(HttpResponse::Ok().json(&response))
}

#[cfg_attr(feature = "openapi", utoipa::path(
    get,
    path = "/api/v1/tasks",
    tag = "tasks",
    responses(
        (status = 200, description = "List of tasks", body = ApiResponse<Vec<Task>>)
    )
))]
pub async fn list_tasks() -> Result<HttpResponse, ApiError> {
    let response = serde_json::json!({"message": "list_tasks endpoint"});
    Ok(HttpResponse::Ok().json(&response))
}

#[cfg_attr(feature = "openapi", utoipa::path(
    put,
    path = "/api/v1/tasks/{id}",
    tag = "tasks",
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    request_body = TaskUpdate,
    responses(
        (status = 200, description = "Task updated successfully", body = ApiResponse<Task>),
        (status = 404, description = "Task not found", body = ApiResponse<()>),
        (status = 400, description = "Invalid request", body = ApiResponse<()>)
    )
))]
pub async fn update_task() -> Result<HttpResponse, ApiError> {
    let response = serde_json::json!({"message": "update_task endpoint"});
    Ok(HttpResponse::Ok().json(&response))
}

#[cfg_attr(feature = "openapi", utoipa::path(
    delete,
    path = "/api/v1/tasks/{id}",
    tag = "tasks",
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 204, description = "Task deleted successfully"),
        (status = 404, description = "Task not found", body = ApiResponse<()>)
    )
))]
pub async fn delete_task() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::NoContent().finish())
}
