mod ctx;
mod inputs;
mod jobs;
mod api;
mod server;
mod dto;
mod utils;
use crate::dto::logevent::LogEventChannelMessage;
use crate::inputs::rediswrite::start_redis_reader;
use crate::jobs::rotate::start_scheduler;
use crate::server::start_http;
use crate::utils::quitsignal::wait_for_signal_impl;
use crate::{
    ctx::{appcontext::AppContext, dbmanager::DbManager},
    inputs::writer::start_events_writer_thread,
};
use std::{sync::mpsc};
use dotenv::dotenv;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();
    //.layer(axum::middleware::from_fn(custom_middleware)); // apply custom middleware
    println!("Starting Application #{:?}.", std::thread::current().id());
    let (events_writer, events_reader) = mpsc::channel::<LogEventChannelMessage>();
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
    {
        let _db = DbManager::connect_to_db("data/config.db", "migrations/config").await;
        if _db.is_err() {
            panic!(
                "Failed to open config db with error {:?}",
                _db.err().unwrap()
            );
        };
        ctx.configdb = Some(Arc::new(Mutex::new(_db.unwrap())));
    }
    let _events_writer_thread = start_events_writer_thread(events_reader);
    tokio::task::spawn(start_scheduler());
    tokio::task::spawn(start_http(ctx.clone()));
    tokio::task::spawn(start_redis_reader(ctx.clone()));

    wait_for_signal_impl(events_writer, _events_writer_thread).await;
}

