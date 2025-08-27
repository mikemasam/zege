mod editor;
mod man;
mod manager;
mod util;
mod db;
use crate::{db::lib::DbManager, editor::sql::sql_routes, man::routes::man_routes};
use dotenv::dotenv;
use tokio::sync::Mutex;
use std::sync::{Arc};

use axum::{Extension, Router};
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
        .nest("/editor", sql_routes())
        .nest("/man", man_routes())
        .layer(cors)
        .layer(Extension(Arc::clone(&dbman)));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
