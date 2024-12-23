mod controllers;
mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use crate::models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new());
    
    println!("Server running at http://localhost:8101");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8101")?
    .run()
    .await
}
