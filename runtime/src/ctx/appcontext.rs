#![allow(dead_code, unused_imports, unused_variables)]
use std::env;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use clap::Parser;
use tokio::sync::Mutex;

use crate::ctx::dbmanager::DbManager;
use crate::dto::logevent::{LogEvent, LogEventChannelMessage};
use crate::utils::appargv::AppArgv;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub storage: Option<Arc<Mutex<DbManager>>>,
    pub event_writer: Sender<LogEventChannelMessage>,
    pub appargv: AppArgv,
}
impl AppContext {
    pub fn new(sender: Sender<LogEventChannelMessage>) -> Self {
        AppContext {
            storage: None,
            event_writer: sender,
            appargv: AppArgv::parse(),
        }
    }
}
