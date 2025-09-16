pub mod list_events;

use axum::{routing, Router};

use crate::output::list_events::list_events_route;



pub fn event_output_routes() -> Router {
    Router::new().route("/", routing::get(list_events_route))
}
