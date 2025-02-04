use actix_web::{web, HttpResponse, Responder};
use crate::repositories::user_repository::UserRepository;
use crate::models::user::{CreateUserDto, UpdateUserDto};

pub async fn create_user(
    repo: web::Data<UserRepository>,
    user: web::Json<CreateUserDto>
) -> impl Responder {
    match repo.create(user.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("User berhasil ditambahkan"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_users(repo: web::Data<UserRepository>) -> impl Responder {
    match repo.find_all().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_user(
    repo: web::Data<UserRepository>,
    id: web::Path<i32>
) -> impl Responder {
    match repo.find_by_id(id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User tidak ditemukan"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_user(
    repo: web::Data<UserRepository>,
    id: web::Path<i32>,
    user: web::Json<UpdateUserDto>
) -> impl Responder {
    match repo.update(id.into_inner(), user.into_inner()).await {
        Ok(true) => HttpResponse::Ok().json("User berhasil diupdate"),
        Ok(false) => HttpResponse::NotFound().body("User tidak ditemukan"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_user(
    repo: web::Data<UserRepository>,
    id: web::Path<i32>
) -> impl Responder {
    match repo.delete(id.into_inner()).await {
        Ok(true) => HttpResponse::Ok().json("User berhasil dihapus"),
        Ok(false) => HttpResponse::NotFound().body("User tidak ditemukan"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}