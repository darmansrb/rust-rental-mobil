use actix_web::web;
use crate::handlers::booking_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bookings")
            .route("", web::post().to(booking_handler::create_booking))
            .route("", web::get().to(booking_handler::get_bookings))
            .route("/{id}", web::get().to(booking_handler::get_booking))
            .route("/{id}", web::put().to(booking_handler::update_booking))
            .route("/{id}", web::delete().to(booking_handler::delete_booking))
            .route("/user/{user_id}", web::get().to(booking_handler::get_user_bookings))
    );
}