use actix_web::{web, HttpResponse, Responder};
use crate::controllers::hello;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/", web::get().to(hello))
    );
}
