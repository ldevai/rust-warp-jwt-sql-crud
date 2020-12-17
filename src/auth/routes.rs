// use std::convert::Infallible;
//
// use warp::{Filter, Future, Rejection, Reply};
// use warp::filters::BoxedFilter;
// use warp::filters::path::Exact;
//
// use crate::{auth, environment, WebResult};
// use crate::dto::LoginRequest;
// use crate::environment::Environment;
//
// pub fn routes(env: Environment) -> BoxedFilter<(impl Reply, )> {
//     let login_route = warp::path!("login")
//         .and(warp::post())
//         .and(warp::body::json())
//         .and(environment::with_env(env.clone()))
//         .and_then(auth::handlers::login_handler);
//
//     let register_route = warp::path!("register")
//         .and(warp::post())
//         .and(warp::body::json())
//         .and(environment::with_env(env.clone()))
//         .and_then(auth::handlers::register_handler);
//
//     login_route.or(register_route).boxed()
// }
