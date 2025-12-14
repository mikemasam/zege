mod events;
mod report;
mod auth;
use axum::{Router, routing};

use crate::api::{auth::auth_routes, events::list_events_route, report::report_routes};


pub fn api_routes() -> Router {
    Router::new()
        .route("/events", routing::get(list_events_route))
        .nest("/reports", report_routes())
        .nest("/auth", auth_routes())

}
