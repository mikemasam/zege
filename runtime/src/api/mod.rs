mod events;
mod reportread;
mod reportcreate;
mod reports;
mod report;

use axum::{Router, routing};

use crate::api::{events::list_events_route, report::report_view_route, reportcreate::report_create_route, reportread::report_read_route, reports::report_index_route};

pub fn api_routes() -> Router {
    Router::new()
        .route("/events", routing::get(list_events_route))
        .route("/reports", routing::get(report_index_route))
        .route("/reports", routing::post(report_create_route))
        .route("/reports/:id", routing::get(report_view_route))
        .route("/reports/:id/read", routing::get(report_read_route))

}
