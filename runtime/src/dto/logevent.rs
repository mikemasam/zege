use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEvent {
    pub timestamp: DateTime<FixedOffset>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub event_name: String,
    pub service_name: String,
    pub event_type: Option<String>,
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
    pub data: Option<HashMap<String, serde_json::Value>>,
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
    pub headers: Option<HashMap<String, serde_json::Value>>,
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
    pub response_size_bytes: Option<i64>,
}

pub enum LogEventChannelMessage {
    Data(Box<LogEvent>),
    Shutdown,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ZegeEventRow {
    pub id: i64,
    pub timestamp: DateTime<FixedOffset>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub event_name: String,
    pub event_type: Option<String>,
    pub ui: Option<String>,
    pub service_name: String,
    pub severity: Option<String>,
    pub message: Option<String>,
    pub error_type: Option<String>,
    pub error_message: Option<String>,
    pub stack_trace: Option<String>,
    pub app_instance_id: Option<String>,
    pub build_commit: Option<String>,
    pub build_id: Option<String>,
    pub app_region: Option<String>,
    pub host_region: Option<String>,
    pub service_version: Option<String>,
    pub environment: Option<String>,
    pub hostname: Option<String>,
    pub host_ip: Option<String>,
    pub host_provider: Option<String>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub transaction_id: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub session_id: Option<String>,
    pub http_method: Option<String>,
    pub http_path: Option<String>,
    pub http_url: Option<String>,
    pub http_origin: Option<String>,
    pub http_status: Option<i32>,
    pub http_headers: Option<serde_json::Value>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub request_id: Option<String>,
    pub referrer: Option<String>,
    pub protocol: Option<String>,
    pub response_size_bytes: Option<i64>,
    pub tags: Option<serde_json::Value>,
    pub labels: Option<serde_json::Value>,
    pub data: Option<serde_json::Value>,
}

impl ZegeEventRow {
    pub fn to_event(self) -> LogEvent {
        LogEvent {
            timestamp: self.timestamp,
            created_at: self.created_at,
            event_name: self.event_name,
            event_type: self.event_type,
            ui: self.ui,
            service_name: self.service_name,
            severity: self.severity,
            message: self.message,

            error: Some(ErrorInfo {
                error_type: self.error_type,
                error_message: self.error_message,
                stack_trace: self.stack_trace,
            }),

            app: Some(AppInfo {
                instance_id: self.app_instance_id,
                build_commit: self.build_commit,
                build_id: self.build_id,
                region: self.app_region,
            }),

            service: Some(ServiceInfo {
                version: self.service_version,
                environment: self.environment,
            }),

            host: Some(HostInfo {
                hostname: self.hostname,
                host_ip: self.host_ip,
                region: self.host_region, // ⚠️ you have no `host_region` field — using `region`
                provider: self.host_provider,
            }),

            tracing: Some(TracingInfo {
                trace_id: self.trace_id,
                span_id: self.span_id,
                transaction_id: self.transaction_id,
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
                origin: self.http_origin,
                status: self.http_status,
                client_ip: self.client_ip,
                user_agent: self.user_agent,
                headers: Some(
                    serde_json::from_value(self.http_headers.unwrap()).unwrap_or_default(),
                ),
            }),

            request: Some(RequestInfo {
                request_id: self.request_id,
                referrer: self.referrer,
                protocol: self.protocol,
                response_size_bytes: self.response_size_bytes,
            }),
            tags: Some(serde_json::from_value(self.tags.unwrap()).unwrap_or_default()),
            labels: Some(serde_json::from_value(self.labels.unwrap()).unwrap_or_default()),
            data: Some(serde_json::from_value(self.data.unwrap()).unwrap_or_default()),
        }
    }
}
