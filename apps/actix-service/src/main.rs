use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware};
use log::{info, error};
use dotenv::dotenv;
use std::env;

fn init_logger() {
    dotenv().ok(); // Load .env file

    // Get log level from environment variable or default to "info"
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    let level_filter = match log_level.to_lowercase().as_str() {
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info, // Default to "info" level
    };

    // Initialize logger with the configured log level
    env_logger::builder()
        .filter(None, level_filter)
        .init();
}

// New route for GET /
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Rust Actix")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    init_logger();

    // Set port to 8101 directly
    let port: u16 = 8101;

    // Simulate the startup process with logging
    info!("[Actix-Service] Server is starting on port {}", port);

    // Start the server
    match HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // Logging middleware
            .route("/", web::get().to(welcome)) // New route for the root
    })
    .bind(format!("localhost:{}", port)) {
        Ok(server) => {
            info!("[Actix-Service] Server is running on port {}", port);
            server.run().await
        }
        Err(error) => {
            if let Some(error_message) = error.get_ref() {
                error!(
                    "Error starting server: Message: {} | Stack: {}",
                    error_message,
                    error
                );
            } else {
                error!("Error starting server: {}", error);
            }
            Err(error)
        }
    }
}
