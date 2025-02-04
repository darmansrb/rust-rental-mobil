use actix_web::web;
use crate::handlers::payment_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .route("", web::post().to(payment_handler::create_payment))
            .route("", web::get().to(payment_handler::get_payments))
            .route("/{id}", web::get().to(payment_handler::get_payment))
            .route("/{id}/status", web::put().to(payment_handler::update_payment_status))
            .route("/{id}", web::delete().to(payment_handler::delete_payment))
            .route("/booking/{booking_id}", web::get().to(payment_handler::get_payments_by_booking))
            .route("/user/{user_id}", web::get().to(payment_handler::get_payments_by_user))
    );
}