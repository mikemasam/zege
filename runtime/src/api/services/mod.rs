use axum::{Router, routing};
use crate::api::services::{create::services_create_route, list::services_index_route};

mod create;
mod list;

pub fn services_routes() -> Router {
    Router::new()
        .route("/", routing::get(services_index_route))
        .route("/", routing::post(services_create_route))
    //.route("/:id", routing::get(report_view_route))
    //.route("/:id/read", routing::get(report_read_route))
}
