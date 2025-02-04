use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminLog {
    pub id: Option<i32>,
    pub admin_id: i32,
    pub action: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAdminLogDto {
    pub admin_id: i32,
    pub action: String,
}