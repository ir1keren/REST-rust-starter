mod config;
mod controllers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod views;

use ntex::web::{self, middleware::Logger, App, HttpServer};
use std::sync::Arc;
use tracing::info;

use config::Config;
use middleware::{cors_middleware, init_logging};
use ntex_remove_trailing_slash::RemoveTrailingSlash;
use repositories::{ProjectRepository, TaskRepository};
use routes::configure_routes;
use services::{ProjectService, TaskService};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    init_logging();

    // Load configuration
    let config = Config::from_env();
    info!("Starting server on {}", config.address());

    // Initialize repositories
    let project_repository = Arc::new(ProjectRepository::new());
    let task_repository = Arc::new(TaskRepository::new());

    // Initialize services
    let project_service = Arc::new(ProjectService::new(project_repository.clone()));
    let task_service = Arc::new(TaskService::new(task_repository, project_repository));

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(cors_middleware())
            .wrap(RemoveTrailingSlash::default())
            .configure(configure_routes)
    })
    .bind(&config.address())?
    .run()
    .await
}
