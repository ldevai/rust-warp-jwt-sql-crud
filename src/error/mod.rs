use std::convert::Infallible;

use serde::Serialize;
use thiserror::Error;
use warp::{http::StatusCode, Reply};

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("could not hash password")]
    ArgonError,
}

#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation failed")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

impl warp::reject::Reject for AuthError {}

impl warp::reject::Reject for AppError {}

impl From<sqlx::error::Error> for AppError {
    fn from(_err: sqlx::error::Error) -> Self {
        AppError::WrongCredentialsError
    }
}

impl From<anyhow::Error> for AppError {
    fn from(_err: anyhow::Error) -> Self {
        AppError::from(_err)
    }
}

impl From<AuthError> for AppError {
    fn from(_err: AuthError) -> Self {
        AppError::from(_err)
    }
}

pub async fn error_handler(err: warp::reject::Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<AppError>() {
        match e {
            AppError::WrongCredentialsError => (StatusCode::FORBIDDEN, e.to_string()),
            AppError::NoPermissionError => (StatusCode::UNAUTHORIZED, e.to_string()),
            AppError::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
            AppError::JWTTokenCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
        )
    } else if let Some(e) = err.find::<AuthError>() {
        println!("AUTH FAILED! {:?}", err);
        match e {
            AuthError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Authentication failed".to_string()),
            _ => (StatusCode::BAD_REQUEST, e.to_string())
        }
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
