// src/controllers/example.rs
use actix_web::{HttpResponse, Responder};

pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hello Users actix-web!")
}
