mod login;
mod papers;
mod signup;
mod switchorg;
use axum::{Router, routing};

use crate::api::auth::{
    login::auth_login, papers::papers_please, signup::auth_signup, switchorg::switch_organization,
};

pub fn auth_public_routes() -> Router {
    Router::new()
        .route("/signup", routing::post(auth_signup))
        .route("/login", routing::post(auth_login))
        .route("/papers-please", routing::get(papers_please))
        .route("/me", routing::get(papers_please))
}
pub fn auth_private_routes() -> Router {
    Router::new().route("/switch-organization", routing::post(switch_organization))
}
