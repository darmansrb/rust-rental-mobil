use sqlx::mysql::MySqlPool;
use crate::models::booking::{self, Booking, BookingApiResponse, CreateBookingDto, UpdateBookingDto};
use rust_decimal::Decimal;
use chrono::NaiveDate;

pub struct BookingRepository {
    pool: MySqlPool,
}

impl BookingRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, dto: CreateBookingDto) -> Result<Booking, sqlx::Error> {
        // Hitung total_price berdasarkan price_per_day dan durasi
        let total_price: Decimal = sqlx::query!(
            r#"
            SELECT price_per_day 
            FROM cars 
            WHERE id = ?
            "#,
            dto.car_id
        )
        .fetch_one(&self.pool)
        .await?
        .price_per_day;

        let result = sqlx::query!(
            r#"
            INSERT INTO bookings (user_id, car_id, start_date, end_date, total_price, status)
            VALUES (?, ?, ?, ?, ?, 'Pending')
            "#,
            dto.user_id,
            dto.car_id,
            dto.start_date,
            dto.end_date,
            total_price
        )
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i32;

        // Ambil data booking yang baru dibuat
        let booking = sqlx::query_as!(
            Booking,
            r#"
            SELECT 
                id, user_id, car_id, start_date as "start_date: NaiveDate", end_date as "end_date: NaiveDate",
                total_price,
                status
            FROM bookings 
            WHERE id = ?
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(booking)
    }

    // pub async fn find_all(&self) -> Result<Vec<BookingApiResponse>, sqlx::Error> {
    //     sqlx::query_as!(
    //         Booking,
    //         r#"
    //         SELECT 
    //             id, user_id, car_id, start_date as "start_date: NaiveDate", end_date as "end_date: NaiveDate",
    //             total_price,
    //             status
    //         FROM bookings
    //         "#
    //     )
    //     .fetch_all(&self.pool)
    //     .await
    // }

    pub async fn find_all(&self) -> Result<Vec<BookingApiResponse>, sqlx::Error> {
        sqlx::query_as!(
            BookingApiResponse,
            r#"
            SELECT 
                b.id, 
                b.user_id, 
                b.car_id, 
                b.start_date as "start_date: NaiveDate", 
                b.end_date as "end_date: NaiveDate",
                b.total_price,
                b.status,
                u.name as "user_name",
                c.name as "car_name"
            FROM bookings b
            INNER JOIN users u ON b.user_id = u.id
            INNER JOIN cars c ON b.car_id = c.id
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Booking>, sqlx::Error> {
        sqlx::query_as!(
            Booking,
            r#"
            SELECT 
                id, user_id, car_id, start_date as "start_date: NaiveDate", end_date as "end_date: NaiveDate",
                total_price,
                status
            FROM bookings 
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update(&self, id: i32, dto: UpdateBookingDto) -> Result<bool, sqlx::Error> {
        let mut query = String::from("UPDATE bookings SET ");
        let mut values = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(start_date) = dto.start_date {
            values.push("start_date = ?".to_string());
            params.push(start_date.to_string());
        }

        if let Some(end_date) = dto.end_date {
            values.push("end_date = ?".to_string());
            params.push(end_date.to_string());
        }

        if let Some(status) = dto.status {
            values.push("status = ?".to_string());
            params.push(status.to_string());
        }

        if values.is_empty() {
            return Ok(false);
        }

        query.push_str(&values.join(", "));
        query.push_str(" WHERE id = ?");

        let mut query_builder = sqlx::query(&query);

        // Bind all parameters
        for param in params {
            query_builder = query_builder.bind(param);
        }

        // Bind the ID parameter
        query_builder = query_builder.bind(id);

        let result = query_builder.execute(&self.pool).await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM bookings WHERE id = ?",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // Additional utility methods
    pub async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Booking>, sqlx::Error> {
        sqlx::query_as!(
            Booking,
            r#"
            SELECT 
                id, user_id, car_id, start_date as "start_date: NaiveDate", end_date as "end_date: NaiveDate",
                total_price,
                status
            FROM bookings 
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update_status(&self, id: i32, status: String) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE bookings SET status = ? WHERE id = ?",
            status.to_string(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}