mod config;
mod models;
mod handlers;
mod routes;
mod repositories;
mod services;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use crate::{
    repositories::{
        user_repository::UserRepository,
        car_repository::CarRepository,
        booking_repository::BookingRepository,
        payment_repository::PaymentRepository,
    },
    services::auth_service::AuthService,
    };

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    let pool = config::database::create_pool().await;
    let jwt_secret = config::jwt::init();
    let user_repository = web::Data::new(UserRepository::new(pool.clone()));
    let car_repository = web::Data::new(CarRepository::new(pool.clone()));
    let booking_repository = web::Data::new(BookingRepository::new(pool.clone()));
    let payment_repository = web::Data::new(PaymentRepository::new(pool.clone()));
    let auth_service = web::Data::new(AuthService::new(pool, jwt_secret));

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                .allowed_origin("http://localhost:5173") // Allow your frontend's origin
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Allow specific HTTP methods
                .allowed_headers(vec!["Content-Type", "Authorization"]) // Allow specific headers
                .supports_credentials() // If you need credentials (cookies, etc.)
            )
            .app_data(user_repository.clone())
            .app_data(car_repository.clone())
            .app_data(booking_repository.clone())
            .app_data(payment_repository.clone())
            .app_data(auth_service.clone())
            .configure(routes::user_routes::config)
            .configure(routes::car_routes::config)
            .configure(routes::booking_routes::config)
            .configure(routes::payment_routes::config)
            .configure(routes::auth_routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}