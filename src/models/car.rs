use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

// #[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
// #[sqlx(rename_all = "lowercase")]
// pub enum String {
//     Available,
//     Rented,
//     Maintenance,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Car {
    pub id: Option<i32>,
    pub name: String,
    pub brand: String,
    pub year: i32,
    pub license_plate: String,
    pub seats: i32,
    pub price_per_day: Decimal,
    pub status: String,
    pub image_url: Option<String>,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCarDto {
    pub name: String,
    pub brand: String,
    pub year: i32,
    pub license_plate: String,
    pub seats: i32,
    pub price_per_day: Decimal,
    pub status: String,
    pub image_url: Option<String>,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCarDto {
    pub name: Option<String>,
    pub brand: Option<String>,
    pub year: Option<i32>,
    pub license_plate: Option<String>,
    pub seats: Option<i32>,
    pub price_per_day: Option<Decimal>,
    pub status: Option<String>,
    pub image_url: Option<String>,
}