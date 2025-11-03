mod config;
mod controllers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod views;

use ntex::web::{middleware::Logger, App, HttpServer};
use tracing::info;

use config::Config;
use middleware::{cors_middleware, init_logging};
use ntex_remove_trailing_slash::RemoveTrailingSlash;
use routes::configure_routes;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    init_logging();

    // Load configuration
    let config = Config::from_env();
    info!("Starting server on {}", config.address());

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
