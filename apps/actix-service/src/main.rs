mod controllers;
mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use crate::models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new());
    
    let port = 8101;
    let address = format!("127.0.0.1:{}", port);

    println!("[Actix-Service] Server is running on port {}", port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::config)
    })
    .bind(address)?
    .run()
    .await
}
