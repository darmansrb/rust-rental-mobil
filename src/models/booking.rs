use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;

// #[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
// #[sqlx(type_name = "booking_status", rename_all = "lowercase")]
// pub enum BookingStatus {
//     Pending,
//     Confirmed,
//     Cancelled,
//     Completed,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Booking {
    pub id: Option<i32>,
    pub user_id: i32,
    pub car_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_price: Decimal,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingDto {
    pub user_id: i32,
    pub car_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookingDto {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookingApiResponse {
    pub id: Option<i32>,
    pub user_id: i32,
    pub car_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_price: Decimal,
    pub status: String,
    pub user_name: Option<String>,   // Added field for user name
    pub car_name: Option<String>,   // Added field for car name
}