use actix_web::{web, HttpResponse, Responder};
use crate::repositories::booking_repository::BookingRepository;
use crate::models::booking::{CreateBookingDto, UpdateBookingDto};

pub async fn create_booking(
    repo: web::Data<BookingRepository>,
    booking: web::Json<CreateBookingDto>,
) -> impl Responder {
    println!("Creating new booking");
    match repo.create(booking.into_inner()).await {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(e) => {
            println!("Error creating booking: {:?}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub async fn get_bookings(repo: web::Data<BookingRepository>) -> impl Responder {
    match repo.find_all().await {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_booking(
    repo: web::Data<BookingRepository>,
    id: web::Path<i32>,
) -> impl Responder {
    match repo.find_by_id(id.into_inner()).await {
        Ok(Some(booking)) => HttpResponse::Ok().json(booking),
        Ok(None) => HttpResponse::NotFound().body("Booking not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_booking(
    repo: web::Data<BookingRepository>,
    id: web::Path<i32>,
    booking: web::Json<UpdateBookingDto>,
) -> impl Responder {
    match repo.update(id.into_inner(), booking.into_inner()).await {
        Ok(true) => HttpResponse::Ok().json("Booking updated successfully"),
        Ok(false) => HttpResponse::NotFound().body("Booking not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_booking(
    repo: web::Data<BookingRepository>,
    id: web::Path<i32>,
) -> impl Responder {
    match repo.delete(id.into_inner()).await {
        Ok(true) => HttpResponse::Ok().json("Booking deleted successfully"),
        Ok(false) => HttpResponse::NotFound().body("Booking not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_user_bookings(
    repo: web::Data<BookingRepository>,
    user_id: web::Path<i32>,
) -> impl Responder {
    match repo.find_by_user_id(user_id.into_inner()).await {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}