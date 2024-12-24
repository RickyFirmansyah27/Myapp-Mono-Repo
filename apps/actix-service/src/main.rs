use actix_web::{web, App, HttpServer};

mod controllers;
mod models;
mod routes;

use crate::models::app_model::AppState;
use crate::routes::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new());

    let port = 8101;
    let address = format!("127.0.0.1:{}", port);

    println!("[Actix-Service] Server is running on http://{}", address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(index::config)
    })
    .bind(&address)?
    .run()
    .await
}
