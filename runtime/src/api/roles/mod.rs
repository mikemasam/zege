use axum::{Router, routing};

use crate::api::roles::list::roles_index_route;
mod list;

pub fn roles_routes() -> Router {
    Router::new().route("/", routing::get(roles_index_route))
}
