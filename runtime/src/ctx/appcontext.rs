#![allow(dead_code)]
use std::env;
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::ctx::dbmanager::DbManager;
use crate::event::event::LogEvent;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub eventsdb: Option<Arc<Mutex<DbManager>>>,
    pub event_writer: Sender<LogEvent>
}
impl AppContext {
    pub fn new(sender: Sender<LogEvent>) -> Self {
        AppContext {  eventsdb: None, event_writer: sender }
    }
}
