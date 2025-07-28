
pub mod util; 
use std::sync::Arc;
use crate::util::db::{DbManager, QueryResult};
use tokio::sync::Mutex;
use dotenv::dotenv;

use axum::{Extension, Json, Router, routing};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    //.layer(axum::middleware::from_fn(custom_middleware)); // apply custom middleware
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{addr}");

    let dbman = Arc::new(Mutex::new(DbManager::new()));
    let app = Router::new()
        .route("/query", routing::post(sql_route))
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
async fn sql_route(
    Extension(dbman): Extension<Arc<Mutex<DbManager>>>,
    Json(payload): Json<InputData>,
) -> Json<QueryResult> {
    let mut db = dbman.lock().await;
    if db.connections.is_empty() {
        db.add_sample_connection();
    }
    let r1 = db.exec(payload.connection_id, payload.sql).await.unwrap();
    Json(r1)
}
