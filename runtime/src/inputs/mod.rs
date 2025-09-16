use axum::{routing, Router};

use crate::inputs::httpwrite::event_route;

pub mod httpwrite;

pub fn event_input_routes() -> Router {
    Router::new().route("/basic", routing::post(event_route))
}
