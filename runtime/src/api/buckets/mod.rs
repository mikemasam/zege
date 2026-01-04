use axum::{Router, routing};
use crate::api::buckets::{create::buckets_create_route, list::buckets_index_route};

mod create;
mod list;

pub fn buckets_routes() -> Router {
    Router::new()
        .route("/", routing::get(buckets_index_route))
        .route("/", routing::post(buckets_create_route))
    //.route("/:id", routing::get(report_view_route))
    //.route("/:id/read", routing::get(report_read_route))
}
