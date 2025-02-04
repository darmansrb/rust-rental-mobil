use actix_web::{web, HttpResponse, Responder};
use crate::{models::auth::LoginDto, services::auth_service::AuthService};

pub async fn login(
    service: web::Data<AuthService>,
    login_dto: web::Json<LoginDto>,
) -> impl Responder {
    match service.login(login_dto.into_inner()).await {
        Ok(Some(response)) => HttpResponse::Ok().json(response),
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}