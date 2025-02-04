use actix_web::{web, HttpResponse, Responder};
use rust_decimal::Decimal;
use crate::repositories::car_repository::CarRepository;
use crate::models::car::{CreateCarDto, UpdateCarDto};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    rc: String,
    pesan: String,
}

pub async fn create_car(
    repo: web::Data<CarRepository>,
    car: web::Json<CreateCarDto>,
) -> impl Responder {
    println!("Creating new car");

    // Validate the car data
    if car.name.is_empty() || car.year <= 0 || car.seats <= 0 || car.price_per_day <= Decimal::new(0, 0) {
        let response = ApiResponse {
            rc: "02".to_string(),
            pesan: "Data tidak lengkap atau tidak valid".to_string(),
        };
        return HttpResponse::BadRequest().json(response);
    }

    match repo.create(car.into_inner()).await {
        // Ok(car) => HttpResponse::Ok().json(car),
        Ok(car) => {
            println!("Sukses menambahkan data");
            // Successfully created the car, return a custom JSON response
            let response = ApiResponse {
                rc: "00".to_string(),
                pesan: "Sukses menambahkan data".to_string(),
            };

            HttpResponse::Created()
                .json(response) // Respond with HTTP 201 Created and the custom response
        }
        Err(e) => {
            println!("Error creating car: {:?}", e);
            let response = ApiResponse {
                rc: "01".to_string(),
                pesan: format!("Error: {}", e),
            };
            // HttpResponse::InternalServerError().body(e.to_string())
            HttpResponse::InternalServerError()
                .json(response) // Respond with HTTP 500 InternalServerError and the error message
        }
    }
}

pub async fn get_cars(repo: web::Data<CarRepository>) -> impl Responder {
    match repo.find_all().await {
        Ok(cars) => HttpResponse::Ok().json(cars),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_car(
    repo: web::Data<CarRepository>,
    id: web::Path<i32>,
) -> impl Responder {
    match repo.find_by_id(id.into_inner()).await {
        Ok(Some(car)) => HttpResponse::Ok().json(car),
        Ok(None) => HttpResponse::NotFound().body("Car not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_car(
    repo: web::Data<CarRepository>,
    id: web::Path<i32>,
    car: web::Json<UpdateCarDto>,
) -> impl Responder {
    match repo.update(id.into_inner(), car.into_inner()).await {
        // Ok(true) => HttpResponse::Ok().json("Car updated successfully"),
        // Ok(false) => HttpResponse::NotFound().body("Car not found"),
        // Err(e) => HttpResponse::InternalServerError().body(e.to_string()),

        Ok(true) => {
            println!("Sukses update data");
            // Successfully created the car, return a custom JSON response
            let response = ApiResponse {
                rc: "00".to_string(),
                pesan: "Sukses update data".to_string(),
            };

            HttpResponse::Created()
                .json(response) // Respond with HTTP 201 Created and the custom response
        }
        Ok(false) => {
            println!("Data tidak di temukan");
            let response = ApiResponse {
                rc: "03".to_string(),
                pesan: "Data tidak di temukan".to_string(),
            };

            HttpResponse::NotFound()
                .json(response) // Respond with HTTP 201 Created and the custom response
        }
        Err(e) => {
            println!("Error update car: {:?}", e);
            let response = ApiResponse {
                rc: "01".to_string(),
                pesan: format!("Error: {}", e),
            };
            // HttpResponse::InternalServerError().body(e.to_string())
            HttpResponse::InternalServerError()
                .json(response) // Respond with HTTP 500 InternalServerError and the error message
        }
    }
}

pub async fn delete_car(
    repo: web::Data<CarRepository>,
    id: web::Path<i32>,
) -> impl Responder {
    match repo.delete(id.into_inner()).await {
        // Ok(true) => HttpResponse::Ok().json("Car deleted successfully"),
        // Ok(false) => HttpResponse::NotFound().body("Car not found"),
        // Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(true) => {
            println!("Sukses hapus data");
            // Successfully delete the car, return a custom JSON response
            let response = ApiResponse {
                rc: "00".to_string(),
                pesan: "Sukses hapus data".to_string(),
            };

            HttpResponse::Created()
                .json(response) // Respond with HTTP 201 Created and the custom response
        }
        Ok(false) => {
            println!("Data tidak di temukan");
            let response = ApiResponse {
                rc: "03".to_string(),
                pesan: "Data tidak di temukan".to_string(),
            };

            HttpResponse::NotFound()
                .json(response) // Respond with HTTP 201 Created and the custom response
        }
        Err(e) => {
            println!("Error delete car: {:?}", e);
            let response = ApiResponse {
                rc: "01".to_string(),
                pesan: format!("Error: {}", e),
            };
            // HttpResponse::InternalServerError().body(e.to_string())
            HttpResponse::InternalServerError()
                .json(response) // Respond with HTTP 500 InternalServerError and the error message
        }
    }
}

pub async fn get_available_cars(repo: web::Data<CarRepository>) -> impl Responder {
    match repo.find_available_cars().await {
        Ok(cars) => HttpResponse::Ok().json(cars),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}