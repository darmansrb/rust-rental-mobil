use actix_web::{web, HttpResponse, Responder};
use crate::repositories::payment_repository::PaymentRepository;
use crate::models::payment::{CreatePaymentDto, UpdatePaymentDto};

pub async fn create_payment(
    repo: web::Data<PaymentRepository>,
    payment: web::Json<CreatePaymentDto>,
) -> impl Responder {
    println!("Creating new payment");
    match repo.create(payment.into_inner()).await {
        Ok(payment) => HttpResponse::Ok().json(payment),
        Err(e) => {
            println!("Error creating payment: {:?}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub async fn get_payments(repo: web::Data<PaymentRepository>) -> impl Responder {
    match repo.find_all().await {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_payment(
    repo: web::Data<PaymentRepository>,
    id: web::Path<i32>,
) -> impl Responder {
    match repo.find_by_id(id.into_inner()).await {
        Ok(Some(payment)) => HttpResponse::Ok().json(payment),
        Ok(None) => HttpResponse::NotFound().body("Payment not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_payment_status(
    repo: web::Data<PaymentRepository>,
    id: web::Path<i32>,
    payment: web::Json<UpdatePaymentDto>,
) -> impl Responder {
    match repo.update_status(id.into_inner(), payment.into_inner()).await {
        Ok(true) => HttpResponse::Ok().json("Payment status updated successfully"),
        Ok(false) => HttpResponse::NotFound().body("Payment not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_payment(
    repo: web::Data<PaymentRepository>,
    id: web::Path<i32>,
) -> impl Responder {
    match repo.delete(id.into_inner()).await {
        Ok(true) => HttpResponse::Ok().json("Payment deleted successfully"),
        Ok(false) => HttpResponse::NotFound().body("Payment not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_payments_by_booking(
    repo: web::Data<PaymentRepository>,
    booking_id: web::Path<i32>,
) -> impl Responder {
    match repo.find_by_booking_id(booking_id.into_inner()).await {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_payments_by_user(
    repo: web::Data<PaymentRepository>,
    user_id: web::Path<i32>,
) -> impl Responder {
    match repo.find_by_user_id(user_id.into_inner()).await {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}