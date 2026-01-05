use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::dto::logevent::TracingInfo;

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEventInput {
    pub bucket_key: Option<String>,
    pub event_organization_id: Option<i64>,
    pub event_bucket_id: Option<i64>,
    pub timestamp: DateTime<FixedOffset>,
    pub service: String,
    pub version: Option<String>,
    pub host: Option<String>,
    pub environment: Option<String>,
    pub event_name: String,
    pub event_type: Option<String>,
    pub message: Option<String>,
    pub tracing: Option<TracingInfo>,
    pub meta: Option<Meta>,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    pub jwt: Option<String>,
}

pub enum LogEventChannelMessage {
    Data(Box<LogEventInput>),
    Shutdown,
}
