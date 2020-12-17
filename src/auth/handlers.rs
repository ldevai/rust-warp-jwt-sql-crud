use serde_json::json;
use warp::Reply;

use crate::{users, WebResult};
use crate::dto::login::LoginRequest;
use crate::environment::Environment;
use crate::error::AuthError;
use crate::auth::{Role, create_jwt};
use crate::users::User;

pub async fn register_handler(req: LoginRequest, env: Environment) -> WebResult<impl Reply> {
    let hash = env.argon().hasher().with_password(&req.password).hash().unwrap();
    let role = Role::User.to_string();
    users::create_user(env.db(), uuid::Uuid::new_v4(), &req.email, &hash, &role).await;
    Ok(warp::reply::json(&json!({"status": "success"})))
}

pub async fn login_handler(req: LoginRequest, env: Environment) -> WebResult<impl Reply> {
    let result = crate::users::get_user(env.db(), &req.email)
        .await?;
    let user = match result {
        Some(_) => result.unwrap(),
        None => return Err(warp::reject::custom(AuthError::ArgonError)),
    };

    let is_valid = env
        .argon()
        .verifier()
        .with_hash(&user.password)
        .with_password(&req.password)
        .verify()
        .or(Err(warp::reject::custom(AuthError::ArgonError)))?;

    if !is_valid {
        return Err(warp::reject::custom(AuthError::InvalidCredentials));
    }

    let token = create_jwt(&user.id.to_string(), &Role::from_str(&user.role)).unwrap();
    return Ok(warp::reply::json(&json!({ "jwt": token })));
}
