use axum::{routing, Router};

use crate::api::auth::signup::{auth_login, auth_signup};

mod signup;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/signup", routing::post(auth_signup))
        .route("/login", routing::post(auth_login))

}
