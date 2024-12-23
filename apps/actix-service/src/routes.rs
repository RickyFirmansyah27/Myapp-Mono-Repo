use actix_web::{web};
use crate::controllers::hello;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/", web::get().to(hello))
    );
}
