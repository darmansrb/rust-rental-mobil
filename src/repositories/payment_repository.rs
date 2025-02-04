use sqlx::mysql::MySqlPool;
use crate::models::payment::{Payment, CreatePaymentDto, UpdatePaymentDto};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

pub struct PaymentRepository {
    pool: MySqlPool,
}

impl PaymentRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, dto: CreatePaymentDto) -> Result<Payment, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO payments (booking_id, user_id, amount, payment_method, status, transaction_date)
            VALUES (?, ?, ?, ?, 'pending', NOW())
            "#,
            dto.booking_id,
            dto.user_id,
            dto.amount,
            dto.payment_method,
        )
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i32;

        // Fetch the created payment
        let payment = sqlx::query_as!(
            Payment,
            r#"
            SELECT 
                id, booking_id, user_id,
                amount,
                payment_method,
                status,
                transaction_date as "transaction_date: DateTime<Utc>"
            FROM payments 
            WHERE id = ?
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(payment)
    }

    pub async fn find_all(&self) -> Result<Vec<Payment>, sqlx::Error> {
        sqlx::query_as!(
            Payment,
            r#"
            SELECT 
                id, booking_id, user_id,
                amount,
                payment_method,
                status ,
                transaction_date as "transaction_date: DateTime<Utc>"
            FROM payments
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Payment>, sqlx::Error> {
        sqlx::query_as!(
            Payment,
            r#"
            SELECT 
                id, booking_id, user_id,
                amount,
                payment_method,
                status,
                transaction_date as "transaction_date: DateTime<Utc>"
            FROM payments 
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update_status(&self, id: i32, dto: UpdatePaymentDto) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE payments SET status = ? WHERE id = ?",
            dto.status.to_string(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM payments WHERE id = ?",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // Additional utility methods
    pub async fn find_by_booking_id(&self, booking_id: i32) -> Result<Vec<Payment>, sqlx::Error> {
        sqlx::query_as!(
            Payment,
            r#"
            SELECT 
                id, booking_id, user_id,
                amount,
                payment_method,
                status,
                transaction_date as "transaction_date: DateTime<Utc>"
            FROM payments 
            WHERE booking_id = ?
            "#,
            booking_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Payment>, sqlx::Error> {
        sqlx::query_as!(
            Payment,
            r#"
            SELECT 
                id, booking_id, user_id,
                amount,
                payment_method,
                status,
                transaction_date as "transaction_date: DateTime<Utc>"
            FROM payments 
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }
}