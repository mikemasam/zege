#![allow(warnings)]
mod api;
mod auth;
mod ctx;
mod dto;
mod inputs;
mod jobs;
mod server;
mod utils;
use crate::ctx::dbmanager::DbManagerConnectOptions;
use crate::dto::logevent::LogEventChannelMessage;
use crate::inputs::rediswrite::start_redis_reader;
use crate::server::start_http;
use crate::utils::appenv::AppLogger;
use crate::utils::daemon::wait_for_signal_impl;
use crate::{
    ctx::{appcontext::AppContext, dbmanager::DbManager},
    inputs::writer::start_events_writer_thread,
};
use clap::Parser;
use dotenv::dotenv;
use sqlx::any::install_default_drivers;
use std::env;
use std::process::{Command, exit};
use std::sync::Arc;
use std::sync::mpsc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();
    install_default_drivers();
    //.layer(axum::middleware::from_fn(custom_middleware)); // apply custom middleware
    let (events_writer, events_reader) = mpsc::channel::<LogEventChannelMessage>();
    let mut ctx = AppContext::new(events_writer.clone());
    if !ctx.appargv.started_as_deamon && ctx.appargv.command.is_none() {
        AppLogger::log(format!("To start an a daemon, use -d",));
        return;
    }
    {
        let _db = DbManager::connect(DbManagerConnectOptions {
            backup: false,
            migrate: true,
        })
        .await;
        if _db.is_err() {
            panic!(
                "Failed to open events db with error {:?}",
                _db.err().unwrap()
            );
        };
        ctx.storage = Some(Arc::new(Mutex::new(_db.unwrap())));
    }
    ctx.appargv.match_commands(ctx.clone()).await.unwrap();
    if !ctx.appargv.started_as_deamon {
        return;
    }
    AppLogger::log(format!(
        "Starting Application #{:?}.",
        std::thread::current().id()
    ));
    let _events_writer_thread = start_events_writer_thread(events_reader);
    //tokio::task::spawn(start_scheduler());
    tokio::task::spawn(start_http(ctx.clone()));
    tokio::task::spawn(start_redis_reader(ctx.clone()));

    wait_for_signal_impl(events_writer, _events_writer_thread).await;
}
