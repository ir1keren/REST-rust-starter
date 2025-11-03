mod config;
mod models;
mod repositories;
mod services;
mod controllers;
mod views;
mod middleware;
mod routes;

use std::sync::Arc;
use ntex::web::{self, App, HttpServer, middleware::Logger};
use tracing::info;

use config::Config;
use repositories::{ProjectRepository, TaskRepository};
use services::{ProjectService, TaskService};
use middleware::{cors_middleware, init_logging};
use routes::configure_routes;

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
            .configure(configure_routes)
    })
    .bind(&config.address())?
    .run()
    .await
}