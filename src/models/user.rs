use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub role: String,
    pub created_at: Option<NaiveDateTime>, 
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub role: String,
    pub created_at: String, 
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub role: String,
    pub created_at: String, 
    pub updated_at: String,
}