use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, query_as_unchecked, query_unchecked};
use uuid;
use warp::Rejection;

use crate::error::{AuthError};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn create_user(
    connection: &PgPool,
    id: uuid::Uuid,
    email: &str,
    password: &str,
    role: &str,
) -> anyhow::Result<u64> {
    query_unchecked!(
        r#"INSERT INTO users (id, email, password, role, created_at) VALUES ($1, $2, $3, $4, $5)"#,
        id,
        email,
        password,
        role,
        Utc::now()
    )
        .execute(connection)
        .await
        .map_err(|e| e.into())
}

pub async fn get_user(connection: &PgPool, email: &str) -> Result<Option<User>, Rejection> {
    let user = query_as_unchecked!(
        User,
        r#"SELECT id, email, password, role, created_at, updated_at FROM users WHERE email = $1"#,
        email
    )
        .fetch_one(connection)
        .await
        .map_err(|e| {
            AuthError::InvalidCredentials
        })
        .ok();
    Ok(user)
}
