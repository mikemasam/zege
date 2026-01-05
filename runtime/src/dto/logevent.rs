use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::lib::buckets::Bucket;

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEvent {
    pub data: Option<HashMap<String, serde_json::Value>>,
    pub ui: Option<String>,
    pub service: String,
    pub host: Option<String>,
    pub version: Option<String>,
    pub timestamp: DateTime<FixedOffset>,
    pub message: Option<String>,
    pub tracing: Option<TracingInfo>,
    pub event_name: String,
    pub event_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TracingInfo {
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub transaction_id: Option<String>,
    pub request_id: Option<String>,
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
    pub host: Option<String>,
    pub version: Option<String>,
    pub event_name: String,
    pub event_type: Option<String>,
    pub message: Option<String>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub transaction_id: Option<String>,
    pub request_id: Option<String>,
    pub data: Option<serde_json::Value>,
}

impl ZegeEventRow {
    pub fn to_event(self) -> LogEvent {
        LogEvent {
            ui: self.event_ui,
            service: self.service,
            version: self.version,
            host: self.host,
            event_name: self.event_name,
            event_type: self.event_type,
            timestamp: self.timestamp,
            message: self.message,
            tracing: Some(TracingInfo {
                trace_id: self.trace_id,
                span_id: self.span_id,
                transaction_id: self.transaction_id,
                request_id: self.request_id,
            }),
            data: Some(serde_json::from_value(self.data.unwrap_or_default()).unwrap_or_default()),
        }
    }
}
