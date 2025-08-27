use std::sync::Arc;

use axum::{Extension, Json, Router, body::Body, response::IntoResponse, routing};
use tokio::sync::Mutex;

use crate::{db::lib::DbManager, editor::lib::{ApiResponse, InputData}};

async fn sql_route(
    Extension(dbman): Extension<Arc<Mutex<DbManager>>>,
    Json(payload): Json<InputData>,
) -> impl IntoResponse {
    let mut db = dbman.lock().await;
    if db.connections.is_empty() {
        db.add_sample_connection();
    }
    match db.exec(payload.connection_id, payload.sql).await {
        Ok(r1) => Json(ApiResponse {
            data: Some(r1),
            status: 200,
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            data: None,
            status: 400,
            error: Some(e.to_string()),
        }),
    }
}
pub fn sql_routes() -> Router<(), Body> {
    let routes: Router<(), Body> = Router::new().route("/query", routing::post(sql_route));
    routes
}
