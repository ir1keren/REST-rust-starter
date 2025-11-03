use crate::controllers::{
    create_project, create_task, delete_project, delete_task, get_project, get_task, health_check,
    list_projects, list_tasks, update_project, update_task,
};
use ntex::web::{self, HttpResponse, ServiceConfig};

#[cfg(feature = "openapi")]
use utoipa::OpenApi;

#[cfg(feature = "openapi")]
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controllers::health_check,
        crate::controllers::create_project,
        crate::controllers::get_project,
        crate::controllers::list_projects,
        crate::controllers::update_project,
        crate::controllers::delete_project,
        crate::controllers::create_task,
        crate::controllers::get_task,
        crate::controllers::list_tasks,
        crate::controllers::update_task,
        crate::controllers::delete_task,
    ),
    components(
        schemas(crate::models::project::Project),
        schemas(crate::models::project::ProjectCreate),
        schemas(crate::models::project::ProjectUpdate),
        schemas(crate::models::task::Task),
        schemas(crate::models::task::TaskCreate),
        schemas(crate::models::task::TaskUpdate),
        schemas(crate::views::api_response::ApiResponse<crate::models::project::Project>),
        schemas(crate::views::api_response::ApiResponse<Vec<crate::models::project::Project>>),
        schemas(crate::views::api_response::ApiResponse<crate::models::task::Task>),
        schemas(crate::views::api_response::ApiResponse<Vec<crate::models::task::Task>>),
        schemas(crate::views::api_response::ErrorResponse),
        schemas(crate::controllers::health_controller::HealthResponse)
    ),
    tags(
        (name = "projects", description = "Project management endpoints"),
        (name = "tasks", description = "Task management endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "Rust MVC API",
        description = "A production-leaning Rust REST API using MVC architecture with ntex framework",
        version = "0.1.0",
        contact(
            name = "API Support",
            email = "support@rust-mvc-api.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server"),
        (url = "https://api.rust-mvc-api.com", description = "Production server")
    )
)]
pub struct ApiDoc;

#[cfg(feature = "openapi")]
async fn openapi_spec() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(&ApiDoc::openapi())
}

#[cfg(feature = "openapi")]
async fn swagger_ui() -> HttpResponse {
    let html = include_str!("../static/swagger-ui.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

pub fn configure_routes(config: &mut ServiceConfig) {
    config
        .service(
            web::scope("/api/v1")
                .service(
                    web::scope("/projects")
                        .route("", web::post().to(create_project))
                        .route("", web::get().to(list_projects))
                        .route("/{id}", web::get().to(get_project))
                        .route("/{id}", web::put().to(update_project))
                        .route("/{id}", web::delete().to(delete_project)),
                )
                .service(
                    web::scope("/tasks")
                        .route("", web::post().to(create_task))
                        .route("", web::get().to(list_tasks))
                        .route("/{id}", web::get().to(get_task))
                        .route("/{id}", web::put().to(update_task))
                        .route("/{id}", web::delete().to(delete_task)),
                ),
        )
        .route("/health", web::get().to(health_check));

    // Add OpenAPI spec endpoint and Swagger UI
    #[cfg(feature = "openapi")]
    {
        config
            .route("/openapi.json", web::get().to(openapi_spec))
            .route("/docs", web::get().to(swagger_ui));
    }
}
