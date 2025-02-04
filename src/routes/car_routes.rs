use actix_web::web;
use crate::handlers::car_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cars")
            .route("", web::post().to(car_handler::create_car))
            .route("", web::get().to(car_handler::get_cars))
            .route("/available", web::get().to(car_handler::get_available_cars))
            .route("/{id}", web::get().to(car_handler::get_car))
            .route("/{id}", web::put().to(car_handler::update_car))
            .route("/{id}", web::delete().to(car_handler::delete_car))
    );
}