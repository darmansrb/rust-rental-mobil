use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Review {
    pub id: Option<i32>,
    pub user_id: i32,
    pub car_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewDto {
    pub user_id: i32,
    pub car_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReviewDto {
    pub rating: Option<i32>,
    pub comment: Option<String>,
}