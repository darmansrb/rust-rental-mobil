use sqlx::mysql::MySqlPool;
use crate::models::user::{User, CreateUserDto, UpdateUserDto};
use chrono::NaiveDateTime;
use bcrypt::{hash, DEFAULT_COST};

pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: CreateUserDto) -> Result<(), sqlx::Error> {

         // Hash password dengan bcrypt
    let hashed_password = hash(user.password.as_bytes(), DEFAULT_COST)
    .map_err(|_| sqlx::Error::Protocol("Failed to hash password".into()))?;

    println!("Inserting user with data:");
    println!("Name: {}", user.name);
    println!("Email: {}", user.email);
    println!("Hashed Password: [HIDDEN]");

        sqlx::query(
            "INSERT INTO users (name, email, password, phone, address, role, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&hashed_password)
        .bind(&user.phone)
        .bind(&user.address)
        .bind(&user.role)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query!(
            r#"
            SELECT 
                id, name, email, password, phone, 
                address, role,
                created_at as "created_at: NaiveDateTime", 
                updated_at as "updated_at: NaiveDateTime"
            FROM users
            "#
        )
        .map(|row| User {
            id: Some(row.id),
            name: row.name,
            email: row.email,
            password: row.password,
            phone: row.phone,
            address: row.address,
            role: row.role,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query!(
            r#"
            SELECT 
                id, name, email, password, phone, 
                address, role,
                created_at as "created_at: NaiveDateTime",
                updated_at as "updated_at: NaiveDateTime"
            FROM users 
            WHERE id = ?
            "#,
            id
        )
        .map(|row| User {
            id: Some(row.id),
            name: row.name,
            email: row.email,
            password: row.password,
            phone: row.phone,
            address: row.address,
            role: row.role,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update(&self, id: i32, user: UpdateUserDto) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE users SET name = ?, email = ? WHERE id = ?"
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM users WHERE id = ?"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}