#![allow(dead_code, unused_imports, unused_variables)]
use std::env;
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::ctx::dbmanager::DbManager;
use crate::dto::logevent::{LogEvent, LogEventChannelMessage};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub eventsdb: Option<Arc<Mutex<DbManager>>>,
    pub configdb: Option<Arc<Mutex<DbManager>>>,
    pub event_writer: Sender<LogEventChannelMessage>
}
impl AppContext {
    pub fn new(sender: Sender<LogEventChannelMessage>) -> Self {
        AppContext {  eventsdb: None, event_writer: sender, configdb: None }
    }
}
