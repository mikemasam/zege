use axum::{routing, Router};

use crate::config::report::report_create_route;

pub mod report;

pub fn report_routes() -> Router {
    Router::new().route("/reports", routing::post(report_create_route))
}
