use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

// #[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
// #[sqlx(rename_all = "snake_case")]
// pub enum PaymentMethod {
//     CreditCard,
//     BankTransfer,
//     Cash,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
// #[sqlx(rename_all = "lowercase")]
// pub enum PaymentStatus {
//     Pending,
//     Paid,
//     Failed,
//     Refunded,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: Option<i32>,
    pub booking_id: i32,
    pub user_id: i32,
    pub amount: Decimal,
    pub payment_method: String,
    pub status: String,
    pub transaction_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaymentDto {
    pub booking_id: i32,
    pub user_id: i32,
    pub amount: Decimal,
    pub payment_method: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePaymentDto {
    pub status: String,
}