#![allow(warnings)]
mod api;
mod ctx;
mod dto;
mod inputs;
mod jobs;
mod lib;
mod server;
mod utils;
use crate::ctx::dbmanager::DbManagerConnectOptions;
use crate::inputs::rediswrite::start_redis_reader;
use crate::lib::events::input::LogEventChannelMessage;
use crate::server::start_http;
use crate::utils::appconfig::{applogger, AppConfig};
use crate::utils::daemon::wait_for_signal_impl;
use crate::{
    ctx::{appcontext::AppContext, dbmanager::DbPoolManager},
    inputs::writer::start_events_writer_thread,
};
use anyhow::Result;
use clap::Parser;
use sqlx::any::install_default_drivers;
use std::env;
use std::process::{Command, exit};
use std::sync::Arc;
use std::sync::mpsc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    AppConfig::init_config()?;
    AppConfig::printlog();
    install_default_drivers();
    //.layer(axum::middleware::from_fn(custom_middleware)); // apply custom middleware
    let (events_writer, events_reader) = mpsc::channel::<LogEventChannelMessage>();
    let _db = DbPoolManager::connect(DbManagerConnectOptions {
        backup: false,
        migrate: true,
    })
    .await;
    let mut ctx = Arc::new(AppContext::new(
        Arc::new(_db.unwrap()),
        events_writer.clone(),
    ));
    if !ctx.appargv.started_as_deamon && ctx.appargv.command.is_none() {
        applogger::log(format!("To start an a daemon, use -d",));
        return Ok(());
    }
    ctx.appargv.match_commands(ctx.clone()).await.unwrap();
    if !ctx.appargv.started_as_deamon {
        return Ok(());
    }
    let _events_writer_thread = start_events_writer_thread(events_reader);
    //tokio::task::spawn(start_scheduler());
    tokio::task::spawn(start_http(ctx.clone()));
    tokio::task::spawn(start_redis_reader(ctx.clone()));

    wait_for_signal_impl(events_writer, _events_writer_thread).await;
    Ok(())
}
