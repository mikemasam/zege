use axum::{Router, routing};

use crate::api::events::{create::event_write_route, list::list_events_route};

mod create;
mod list;

pub fn old_event_input_routes() -> Router {
    Router::new()
        .route("/e/i/basic", routing::post(event_write_route))
        .route("/e/i", routing::post(event_write_route))
}

pub fn events_routes() -> Router {
    Router::new().route("/", routing::get(list_events_route))
}
