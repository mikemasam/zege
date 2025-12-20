use crate::utils::appenv::AppLogger;
use crate::{api::api_routes};
use crate::ctx::appcontext::AppContext;
use crate::inputs::event_input_routes;
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

pub async fn start_http(ctx: AppContext) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3432));
    AppLogger::log("Server running at http://{addr}".to_string());
    let arc_ctx = Arc::new(Mutex::new(ctx));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api/v1/e/i", event_input_routes())
        .nest("/api/v1/", api_routes())
        .layer(cors)
        .layer(Extension(Arc::clone(&arc_ctx)));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
