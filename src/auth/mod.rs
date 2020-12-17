use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::Result;

pub mod handlers;
pub mod routes;
pub mod middleware;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

#[derive(Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}

pub fn create_jwt(uid: &str, role: &Role) -> Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(1800))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
    jsonwebtoken::encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| AppError::JWTTokenError)
}
