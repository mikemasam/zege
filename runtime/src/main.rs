pub mod util;
use crate::util::db::{DbManager, QueryResult};
use dotenv::dotenv;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{
    Extension, Json, Router,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    //.layer(axum::middleware::from_fn(custom_middleware)); // apply custom middleware
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{addr}");

    let dbman = Arc::new(Mutex::new(DbManager::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/query", routing::post(sql_route))
        .layer(cors)
        .layer(Extension(Arc::clone(&dbman)));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
#[derive(Deserialize)]
struct InputData {
    sql: String,
    connection_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse {
    data: Option<QueryResult>,
    error: Option<String>,
    status: i32,
}
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
