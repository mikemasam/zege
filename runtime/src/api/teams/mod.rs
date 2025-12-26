use axum::{Router, routing};

use crate::api::teams::{create::teams_create_route, list::teams_index_route};

mod create;
mod list;

pub fn teams_routes() -> Router {
    Router::new()
        .route("/", routing::get(teams_index_route))
        .route("/", routing::post(teams_create_route))
    //.route("/:id", routing::get(report_view_route))
    //.route("/:id/read", routing::get(report_read_route))
}
