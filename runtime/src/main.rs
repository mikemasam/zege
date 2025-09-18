mod ctx;
mod event;
mod http;
mod inputs;
mod jobs;
mod output;
use axum::{Extension, Router};
use dotenv::dotenv;
use std::process;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{net::SocketAddr, sync::mpsc};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::Mutex;
use tokio::time::{self, Instant, sleep};
use tower_http::cors::{Any, CorsLayer};
use crate::event::writer::LogEventMessage;
use crate::inputs::event_input_routes;
use crate::jobs::rotate::rotate_events;
use crate::output::event_output_routes;
use crate::{
    ctx::{appcontext::AppContext, dbmanager::DbManager},
    event::writer::event_write_worker,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    //.layer(axum::middleware::from_fn(custom_middleware)); // apply custom middleware
    println!("Starting Application #{:?}.", std::thread::current().id());
    let (events_writer, events_reader) = mpsc::channel::<LogEventMessage>();
    let mut ctx = AppContext::new(events_writer.clone());
    {
        let _db = DbManager::connect_to_event_db(true).await;
        if _db.is_err() {
            panic!(
                "Failed to open events db with error {:?}",
                _db.err().unwrap()
            );
        };
        ctx.eventsdb = Some(Arc::new(Mutex::new(_db.unwrap())));
    }
    let _events_writer_thread = start_events_writer_thread(events_reader);
    tokio::task::spawn(start_scheduler());
    tokio::task::spawn(start_http(ctx));
    wait_for_signal_impl(events_writer, _events_writer_thread).await;
}

async fn wait_for_signal_impl(events_writer: Sender<LogEventMessage>, writer_thread: JoinHandle<()>) {
    use tokio::signal::unix;
    let mut signal_terminate = unix::signal(unix::SignalKind::terminate()).unwrap();
    let mut signal_quit = unix::signal(unix::SignalKind::quit()).unwrap();
    let mut signal_interrupt = unix::signal(unix::SignalKind::interrupt()).unwrap();

    //let mut signal_hangup = unix::signal(unix::SignalKind::hangup()).unwrap();
    //_ = signal_hangup.recv() => println!("Received HANGUP."), TODO: keep named pipe open, reload config and continue to run
    tokio::select! {
        _ = signal_terminate.recv() => {},
        _ = signal_quit.recv() => {},
        _ = signal_interrupt.recv() => {},
    };
    let _ = events_writer.send(LogEventMessage::Shutdown);
    drop(events_writer);
    println!("  ...waiting for other threads for 10s");
    tokio::task::spawn(async {
        sleep(Duration::from_secs(10)).await;
        process::exit(0);
    });
    let _ = writer_thread.join();
}

fn start_events_writer_thread(receiver: Receiver<LogEventMessage>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async { event_write_worker(receiver).await });
    })
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

async fn start_scheduler() {
    let mut interval = time::interval(time::Duration::from_secs(60));
    println!(
        "Starting Background Job #{:?}.",
        std::thread::current().id()
    );
    loop {
        interval.tick().await;
        let start_time = Instant::now();
        let _ = tokio::task::spawn_blocking(|| {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                rotate_events().await;
            });
        })
        .await;
        let elapsed_time = start_time.elapsed();
        let workers_count = Handle::current().metrics().num_workers();
        println!("# rotation time: {elapsed_time:?}, workers: {workers_count}");
    }
}
