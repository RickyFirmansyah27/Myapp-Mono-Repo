use actix_web::{web, App, HttpServer};
use crate::helper::http_logger::HttpLogger as MiddlewareLogger;
use crate::helper::logger::Logger as LogtailLogger;
use tracing_subscriber;
use dotenv::dotenv;

mod helper;
mod controllers;
mod models;
mod routes;

use crate::models::app_model::AppState;
use crate::routes::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 
    tracing_subscriber::fmt::init();
    let app_state = web::Data::new(AppState::new());

    let port = 8101;
    let address = format!("0.0.0.0:{}", port);

    let logtail_logger = LogtailLogger::new("Actix-Service".to_string());
    let http_logger = MiddlewareLogger::new(logtail_logger);
    println!("[Actix-Service] Server is running on http://{}", address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(http_logger.clone())
            .configure(index::config)
    })
    .bind(&address)?
    .run()
    .await
}
