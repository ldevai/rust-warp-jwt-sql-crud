use warp::{Filter};

use crate::auth::Role;
use crate::environment::Environment;

mod auth;
mod dto;
mod environment;
mod error;
mod handlers;
mod users;

type Result<T> = std::result::Result<T, error::AppError>;
type WebResult<T> = std::result::Result<T, warp::reject::Rejection>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if dotenv::dotenv().is_err() {
        eprintln!("Error reading .env file in the current folder!");
    }

    let env = Environment::new().await?;

    let login_route = warp::path!("api" / "auth" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and(environment::with_env(env.clone()))
        .and_then(auth::handlers::login_handler);

    let register_route = warp::path!("api" / "auth" / "register")
        .and(warp::post())
        .and(warp::body::json())
        .and(environment::with_env(env.clone()))
        .and_then(auth::handlers::register_handler);

    let auth_routes = login_route.or(register_route);

    let user_route = warp::path!("api" / "user")
        .and(auth::middleware::with_auth(Role::User))
        .and_then(handlers::user_handler);

    let admin_route = warp::path!("api" / "admin")
        .and(auth::middleware::with_auth(Role::Admin))
        .and_then(handlers::admin_handler);

    let routes = auth_routes
        .or(user_route)
        .or(admin_route)
        .recover(error::error_handler);

    let svc = warp::service(routes);

    let make_svc = hyper::service::make_service_fn(|_: _| {
        let svc = svc.clone();
        async move { Ok::<_, std::convert::Infallible>(svc) }
    });

    let server = if let Some(l) = listenfd::ListenFd::from_env().take_tcp_listener(0).unwrap() {
        hyper::server::Server::from_tcp(l).unwrap()
    } else {
        hyper::server::Server::bind(&([127, 0, 0, 1], 8000).into())
    };

    server.serve(make_svc).await?;
    Ok(())
}
