use bcrypt::{verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::{
    auth::{LoginDto, LoginResponse, Claims},
    user::User,
};
use sqlx::mysql::MySqlPool;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::NaiveDateTime;

pub struct AuthService {
    pool: MySqlPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: MySqlPool, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }

    pub async fn login(&self, dto: LoginDto) -> Result<Option<LoginResponse>, Box<dyn std::error::Error>> {
        // Find user by email
        let user = sqlx::query!(
            r#"
            SELECT 
                id, name, email, password, phone, 
                address, role,
                created_at as "created_at: NaiveDateTime",
                updated_at as "updated_at: NaiveDateTime"
            FROM users 
            WHERE email = ?
            "#,
            dto.email
        )
        .fetch_optional(&self.pool)
        .await?;

        match user {
            Some(user) => {
                // Verify password
                if !verify(dto.password, &user.password)? {
                    return Ok(None);
                }

                // Generate JWT token
                let exp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs() as usize + 24 * 3600; // 24 hours from now

                let claims = Claims {
                    sub: user.id,
                    name: user.name.clone(),
                    email: user.email.clone(),
                    role: user.role.to_string(),
                    exp,
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
                )?;

                Ok(Some(LoginResponse {
                    token,
                    user_id: user.id,
                    name: user.name,
                    email: user.email,
                    role: user.role,
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, DEFAULT_COST)
    }
}