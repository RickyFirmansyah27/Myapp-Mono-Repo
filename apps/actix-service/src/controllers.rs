// src/controllers/example.rs
use actix_web::{HttpResponse, Responder};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to actix-web!")
}
