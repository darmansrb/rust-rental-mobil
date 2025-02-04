use sqlx::mysql::MySqlPool;
use crate::models::car::{Car, CreateCarDto, UpdateCarDto};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    rc: String,
    pesan: String,
}

pub struct CarRepository {
  pool: MySqlPool,
}

impl CarRepository {
  pub fn new(pool: MySqlPool) -> Self {
    Self { pool }
  }

  pub async fn create(&self, car: CreateCarDto) -> Result<Car, sqlx::Error> {
    // Insert terlebih dahulu
    let result = sqlx::query(
      "INSERT INTO cars (name, brand, year, license_plate, seats, price_per_day, status, image_url) VALUES (?, ?, ?, ?, ?, ?, 'Available', ?)"
  )
  .bind(&car.name)
  .bind(&car.brand)
  .bind(&car.year)
  .bind(&car.license_plate)
  .bind(&car.seats)
  .bind(&car.price_per_day)
  .bind(&car.image_url)
  .execute(&self.pool)
  .await?;

  // Ambil ID yang baru dibuat
  let id = result.last_insert_id() as i32;

  // Query untuk mengambil data yang baru diinsert
  let car = sqlx::query_as!(
      Car,
      r#"
      SELECT 
          id, name, brand, year, license_plate, seats,
          price_per_day as "price_per_day!",
          status as "status: _",
          image_url
      FROM cars 
      WHERE id = ?
      "#,
      id
  )
  .fetch_one(&self.pool)
  .await?;

  Ok(car)
}

  // pub async fn create(&self, car: CreateCarDto) -> Result<(), sqlx::Error> {
  //   sqlx::query(
  //     "INSERT INTO cars (name, brand, year, license_plate, seats, price_per_day, status, image_url) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
  //   )
  //   .bind(&car.name)
  //   .bind(&car.brand)
  //   .bind(&car.year)
  //   .bind(&car.license_plate)
  //   .bind(&car.seats)
  //   .bind(&car.price_per_day)
  //   .bind(&car.status)
  //   .bind(&car.image_url)
  //   // .bind(&car.created_at)
  //   // .bind(&car.updated_at)
  //   .execute(&self.pool)
  //   .await?;

  //   Ok(())
  // }


  pub async fn find_all(&self) -> Result<Vec<Car>, sqlx::Error> {
    sqlx::query_as!(
        Car,
        r#"
        SELECT 
            id, name, brand, year, license_plate, seats,
            price_per_day as "price_per_day!",
            status as "status: _",
            image_url
        FROM cars
        "#
    )
    .fetch_all(&self.pool)
    .await
}

pub async fn find_by_id(&self, id: i32) -> Result<Option<Car>, sqlx::Error> {
    sqlx::query_as!(
        Car,
        r#"
        SELECT 
            id, name, brand, year, license_plate, seats,
            price_per_day as "price_per_day!",
            status as "status: _",
            image_url
        FROM cars 
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(&self.pool)
    .await
}

pub async fn update(&self, id: i32, dto: UpdateCarDto) -> Result<bool, sqlx::Error> {
    let mut query = String::from("UPDATE cars SET ");
    let mut values = Vec::new();
    let mut params = Vec::new();

    if let Some(name) = dto.name {
        values.push("name = ?".to_string());
        params.push(name);
    }

    if let Some(brand) = dto.brand {
        values.push("brand = ?".to_string());
        params.push(brand);
    }

    if let Some(year) = dto.year {
        values.push("year = ?".to_string());
        params.push(year.to_string());
    }

    if let Some(license_plate) = dto.license_plate {
        values.push("license_plate = ?".to_string());
        params.push(license_plate);
    }

    if let Some(seats) = dto.seats {
        values.push("seats = ?".to_string());
        params.push(seats.to_string());
    }

    if let Some(price_per_day) = dto.price_per_day {
        values.push("price_per_day = ?".to_string());
        params.push(price_per_day.to_string());
    }

    if let Some(status) = dto.status {
        values.push("status = ?".to_string());
        params.push(status.to_string());
    }

    if let Some(image_url) = dto.image_url {
        values.push("image_url = ?".to_string());
        params.push(image_url);
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
        "DELETE FROM cars WHERE id = ?",
        id
    )
    .execute(&self.pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn find_available_cars(&self) -> Result<Vec<Car>, sqlx::Error> {
    sqlx::query_as!(
        Car,
        r#"
        SELECT 
            id, name, brand, year, license_plate, seats,
            price_per_day as "price_per_day!",
            status as "status: _",
            image_url
        FROM cars 
        WHERE status = 'Available'
        "#
    )
    .fetch_all(&self.pool)
    .await
}

pub async fn update_status(&self, id: i32, status: String) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE cars SET status = ? WHERE id = ?",
        status.to_string(),
        id
    )
    .execute(&self.pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

}