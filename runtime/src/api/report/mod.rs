use axum::{Router, routing};

use crate::api::report::{
    create::report_create_route, list::report_index_route, pin::report_pinned_route,
    read::report_read_route, view::report_view_route,
};
mod create;
mod list;
mod pin;
mod read;
mod view;

pub fn report_routes() -> Router {
    Router::new()
        .route("/", routing::get(report_index_route))
        .route("/", routing::post(report_create_route))
        .route("/pin", routing::get(report_pinned_route))
        .route("/:id", routing::get(report_view_route))
        .route("/:id/read", routing::post(report_read_route))
}
