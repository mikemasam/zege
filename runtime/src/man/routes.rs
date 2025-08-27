
use axum::{Router, body::Body, routing};

use crate::man::appconfig::create_connection;

pub fn man_routes() -> Router<(), Body> {
    let routes: Router<(), Body> = Router::new().route("/connections/create", routing::post(create_connection));
    routes
}
