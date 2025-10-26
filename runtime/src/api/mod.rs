mod events;
mod reportcreate;
mod reports;

use axum::{Router, routing};

use crate::api::{events::list_events_route, reportcreate::report_create_route, reports::report_index_route};

pub fn api_routes() -> Router {
    Router::new()
        .route("/events", routing::get(list_events_route))
        .route("/reports", routing::get(report_index_route))
        .route("/reports", routing::post(report_create_route))
}
