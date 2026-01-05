use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::lib::buckets::Bucket;

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEvent {
    pub event_ui: Option<String>,
    pub event_created_at: DateTime<FixedOffset>,
    pub event_name: String,
    pub service: String,
    pub version: Option<String>,
    pub environment: Option<String>,
    pub timestamp: DateTime<FixedOffset>,
    pub event_type: Option<String>,
    pub severity: Option<String>,
    pub message: Option<String>,
    pub error: Option<ErrorInfo>,
    pub app: Option<AppInfo>,
    pub host: Option<HostInfo>,
    pub tracing: Option<TracingInfo>,
    pub user: Option<UserInfo>,
    pub http: Option<HttpInfo>,
    pub tags: Option<Vec<String>>,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HostInfo {
    pub hostname: Option<String>,
    pub host_ip: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TracingInfo {
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub transaction_id: Option<String>,
    pub request_id: Option<String>,
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
    pub status: Option<i32>,
    pub client_ip: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorInfo {
    pub error_type: Option<String>,
    pub stack_trace: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppInfo {
    pub instance_id: Option<String>,
    pub build_commit: Option<String>,
    pub build_id: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ZegeEventRow {
    pub id: i64,
    pub event_ui: Option<String>,
    pub event_organization_id: i64,
    pub event_bucket_id: i64,
    pub event_created_at: DateTime<FixedOffset>,
    pub timestamp: DateTime<FixedOffset>,
    pub service: String,
    pub version: Option<String>,
    pub environment: Option<String>,
    pub event_name: String,
    pub event_type: Option<String>,
    pub severity: Option<String>,
    pub message: Option<String>,
    pub error_type: Option<String>,
    pub stack_trace: Option<String>,
    pub app_instance_id: Option<String>,
    pub build_commit: Option<String>,
    pub build_id: Option<String>,
    pub hostname: Option<String>,
    pub host_ip: Option<String>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub transaction_id: Option<String>,
    pub request_id: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub session_id: Option<String>,
    pub http_method: Option<String>,
    pub http_path: Option<String>,
    pub http_url: Option<String>,
    pub http_status: Option<i32>,
    pub client_ip: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub data: Option<serde_json::Value>,
}

impl ZegeEventRow {
    pub fn to_event(self) -> LogEvent {
        LogEvent {
            event_ui: self.event_ui,
            timestamp: self.timestamp,
            event_created_at: self.event_created_at,
            event_name: self.event_name,
            event_type: self.event_type,
            service: self.service,
            version: self.version,
            environment: self.environment,
            severity: self.severity,
            message: self.message,

            error: Some(ErrorInfo {
                error_type: self.error_type,
                stack_trace: self.stack_trace,
            }),

            app: Some(AppInfo {
                instance_id: self.app_instance_id,
                build_commit: self.build_commit,
                build_id: self.build_id,
            }),

            host: Some(HostInfo {
                hostname: self.hostname,
                host_ip: self.host_ip,
            }),

            tracing: Some(TracingInfo {
                trace_id: self.trace_id,
                span_id: self.span_id,
                transaction_id: self.transaction_id,
                request_id: self.request_id,
            }),

            user: Some(UserInfo {
                id: self.user_id,
                name: self.user_name,
                email: self.user_email,
                session_id: self.session_id,
            }),

            http: Some(HttpInfo {
                method: self.http_method,
                path: self.http_path,
                url: self.http_url,
                status: self.http_status,
                client_ip: self.client_ip,
            }),
            tags: Some(serde_json::from_value(self.tags.unwrap_or_default()).unwrap_or_default()),
            data: Some(serde_json::from_value(self.data.unwrap_or_default()).unwrap_or_default()),
        }
    }
}
