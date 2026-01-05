use crate::appconfig;
use crate::ctx::appcontext::AppContext;
use crate::{api::api_routes, utils::appconfig::applogger};
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::fs::ServeDir;

pub async fn start_http(ctx: Arc<AppContext>) {
    let port = appconfig!().port.unwrap_or(3432);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    applogger::log(format!("Server running at http://{addr}"));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let frontend = ServeDir::new("./static").append_index_html_on_directories(true);

    let app = Router::new()
        .nest("/api/v1/", api_routes())
        .layer(cors)
        .layer(Extension(ctx.clone()))
        .fallback_service(frontend);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
