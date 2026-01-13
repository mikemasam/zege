use axum::{Router, routing};

use crate::api::explore::read::data_read_route;

mod read;

pub fn data_routes() -> Router {
    Router::new()
        .route("/execute", routing::post(data_read_route))
}
