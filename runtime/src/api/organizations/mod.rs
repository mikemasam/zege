use axum::{Router, routing};

use crate::api::organizations::{create::organizations_create_route, list::organizations_index_route};

mod create;
mod list;

pub fn organizations_routes() -> Router {
    Router::new()
        .route("/", routing::get(organizations_index_route))
        .route("/", routing::post(organizations_create_route))
    //.route("/:id", routing::get(report_view_route))
    //.route("/:id/read", routing::get(report_read_route))
}
