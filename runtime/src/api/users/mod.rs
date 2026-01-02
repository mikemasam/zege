use axum::{Router, routing};

use crate::api::users::{create::users_create_route, list::users_index_route};
mod create;
mod list;

pub fn users_routes() -> Router {
    Router::new()
        .route("/", routing::get(users_index_route))
        .route("/", routing::post(users_create_route))
}
