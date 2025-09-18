use std::collections::HashMap;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEvent {
    pub timestamp: DateTime<Utc>,
    pub _time: Option<DateTime<Utc>>,
    pub event_name: String,
    pub service_name: String,
    pub ui: Option<String>,
    pub severity: Option<String>,
    pub message: Option<String>,
    pub error: Option<ErrorInfo>,
    pub app: Option<AppInfo>,
    pub service: Option<ServiceInfo>,
    pub host: Option<HostInfo>,
    pub tracing: Option<TracingInfo>,
    pub user: Option<UserInfo>,
    pub http: Option<HttpInfo>,
    pub request: Option<RequestInfo>,
    pub tags: Option<Vec<String>>,
    pub labels: Option<HashMap<String, String>>,
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceInfo {
    pub version: Option<String>,
    pub environment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HostInfo {
    pub hostname: Option<String>,
    pub host_ip: Option<String>,
    pub region: Option<String>,
    pub provider: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TracingInfo {
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub transaction_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpInfo {
    pub method: Option<String>,
    pub path: Option<String>,
    pub url: Option<String>,
    pub origin: Option<String>,
    pub status: Option<i32>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorInfo {
    pub error_type: Option<String>,
    pub error_message: Option<String>,
    pub stack_trace: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppInfo {
    pub instance_id: Option<String>,
    pub build_commit: Option<String>,
    pub build_id: Option<String>,
    pub region: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestInfo {
    pub request_id: Option<String>,
    pub referrer: Option<String>,
    pub protocol: Option<String>,
    pub response_size_bytes: Option<f64>,
}
