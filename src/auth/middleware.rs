use warp::Filter;
use warp::http::{HeaderMap, HeaderValue};

use crate::{Result, WebResult};
use crate::auth::{Claims, Role, JWT_SECRET, BEARER};
use crate::error::AppError;


pub fn with_auth(role: Role) -> impl Filter<Extract=(String, ), Error=warp::reject::Rejection> + Clone {
    warp::header::headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
        .and_then(authorize)
}

async fn authorize((role, headers): (Role, HeaderMap<HeaderValue>)) -> WebResult<String> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = jsonwebtoken::decode::<Claims>(
                &jwt,
                &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET),
                &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512),
            )
                .map_err(|_| warp::reject::custom(AppError::JWTTokenError))?;

            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(warp::reject::custom(AppError::NoPermissionError));
            }

            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(warp::reject::custom(AppError::from(e))),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(warp::http::header::AUTHORIZATION) {
        Some(v) => v,
        None => return Err(AppError::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(AppError::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(AppError::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
