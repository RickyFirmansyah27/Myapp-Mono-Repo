use actix_web::{web};
use crate::controllers::user_controller::{get_users};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/users", web::get().to(get_users))
    );
}
