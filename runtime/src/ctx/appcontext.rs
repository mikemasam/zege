#![allow(dead_code, unused_imports, unused_variables)]
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use clap::Parser;
use std::env;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use tokio::sync::Mutex;

use crate::ctx::dbmanager::DbPoolManager;
use crate::dto::logevent::LogEvent;
use crate::lib::events::input::LogEventChannelMessage;
use crate::utils::appargv::AppArgv;

pub type DbStorage = Arc<DbPoolManager>;

#[derive(Debug)]
pub struct AppContext {
    pub storage: DbStorage,
    pub event_writer: Sender<LogEventChannelMessage>,
    pub appargv: AppArgv,
}
impl AppContext {
    pub fn new(storage: DbStorage, sender: Sender<LogEventChannelMessage>) -> Self {
        AppContext {
            storage: storage,
            event_writer: sender,
            appargv: AppArgv::parse(),
        }
    }
}
