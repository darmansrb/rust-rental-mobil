use actix_web::web;
use crate::handlers::auth_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(auth_handler::login))
    );
}