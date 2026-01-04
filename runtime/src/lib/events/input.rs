use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::dto::logevent::{AppInfo, ErrorInfo, HostInfo, HttpInfo, ServiceInfo, TracingInfo};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEventInput {
    pub bucket_key: Option<String>,
    pub event_organization_id: Option<i64>,
    pub event_bucket_id: Option<i64>,
    pub timestamp: DateTime<FixedOffset>,
    pub service_name: String,
    pub event_name: String,
    pub event_type: Option<String>,
    pub severity: Option<String>,
    pub message: Option<String>,
    pub error: Option<ErrorInfo>,
    pub app: Option<AppInfo>,
    pub service: Option<ServiceInfo>,
    pub host: Option<HostInfo>,
    pub tracing: Option<TracingInfo>,
    pub user: Option<UserInfoInput>,
    pub http: Option<HttpInfo>,
    pub tags: Option<Vec<String>>,
    pub labels: Option<HashMap<String, String>>,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfoInput {
    pub jwt: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub session_id: Option<String>,
}

pub enum LogEventChannelMessage {
    Data(Box<LogEventInput>),
    Shutdown,
}
