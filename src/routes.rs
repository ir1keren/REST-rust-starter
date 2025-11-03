use ntex::web::{self, ServiceConfig};
use crate::controllers::{
    health_check,
    create_project, get_project, list_projects, update_project, delete_project,
    create_task, get_task, list_tasks, update_task, delete_task,
};

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
                        .route("/{id}", web::delete().to(delete_project))
                )
                .service(
                    web::scope("/tasks")
                        .route("", web::post().to(create_task))
                        .route("", web::get().to(list_tasks))
                        .route("/{id}", web::get().to(get_task))
                        .route("/{id}", web::put().to(update_task))
                        .route("/{id}", web::delete().to(delete_task))
                )
        )
        .route("/health", web::get().to(health_check));
}