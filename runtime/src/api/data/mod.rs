use axum::{Router, routing};

use crate::api::data::read::data_read_route;

mod read;

pub fn data_routes() -> Router {
    Router::new()
        .route("/execute", routing::post(data_read_route))
}
