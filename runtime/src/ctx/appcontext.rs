#![allow(dead_code, unused_imports, unused_variables)]
use std::env;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use tokio::sync::Mutex;

use crate::ctx::dbmanager::DbManager;
use crate::dto::logevent::{LogEvent, LogEventChannelMessage};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub storage: Option<Arc<Mutex<DbManager>>>,
    pub event_writer: Sender<LogEventChannelMessage>,
}
impl AppContext {
    pub fn new(sender: Sender<LogEventChannelMessage>) -> Self {
        AppContext {
            storage: None,
            event_writer: sender,
        }
    }
}

pub struct AppEnv {}

impl AppEnv {
    pub fn log(str: String) {
        println!("LOG: {}", str);
    }
    pub fn debug(str: String) {
        println!("DEBUG: {}", str);
    }
    pub fn error(str: String) {
        println!("ERR: {}", str);
    }
}
