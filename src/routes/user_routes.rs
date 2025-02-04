use actix_web::web;
use crate::handlers::user_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(user_handler::create_user))
            .route("", web::get().to(user_handler::get_users))
            .route("/{id}", web::get().to(user_handler::get_user))
            .route("/{id}", web::put().to(user_handler::update_user))
            .route("/{id}", web::delete().to(user_handler::delete_user))
    );
}