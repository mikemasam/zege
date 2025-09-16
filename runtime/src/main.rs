mod ctx;
mod event;
mod inputs;
mod http;
mod output;
use axum::{Extension, Router};
use dotenv::dotenv;
use tokio::runtime::Runtime;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::{net::SocketAddr, sync::mpsc};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use crate::event::event::LogEvent;
use crate::inputs::event_input_routes;
use crate::output::event_output_routes;
use crate::{
    ctx::{
        appcontext::AppContext,
        dbmanager::DbManager,
    },
    event::{writer::event_write_worker},
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    //.layer(axum::middleware::from_fn(custom_middleware)); // apply custom middleware

    let (events_writer, events_reader) = mpsc::channel::<LogEvent>();
    let mut ctx = AppContext::new(events_writer);
    {
        let _db = DbManager::new("data/events.db", Some("migrations/events")).await;
        if _db.is_err() {
            panic!(
                "Failed to open events db with error {:?}",
                _db.err().unwrap()
            );
        };
        ctx.eventsdb = Some(Arc::new(Mutex::new(_db.unwrap())));
    }
    start_events_thread(events_reader);
    start_http(ctx).await;
}

fn start_events_thread(receiver: Receiver<LogEvent>) {
    std::thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async { event_write_worker(receiver).await });
    });
}

async fn start_http(ctx: AppContext) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3432));
    println!("Server running at http://{addr}");
    let arc_ctx = Arc::new(Mutex::new(ctx));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api/v1/e/i", event_input_routes())
        .nest("/api/v1/events", event_output_routes())
        .layer(cors)
        .layer(Extension(Arc::clone(&arc_ctx)));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

