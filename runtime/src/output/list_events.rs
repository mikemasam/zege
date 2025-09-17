#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::event::event::{
    AppInfo, ErrorInfo, HostInfo, HttpInfo, LogEvent, RequestInfo, ServiceInfo, TracingInfo,
    UserInfo,
};
use crate::http::AppResponse;
use axum::response::IntoResponse;
use axum::{Extension, extract::Query};
use futures::StreamExt;
use serde::Deserialize;
use sqlx::{QueryBuilder, Row};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct Params {
    search: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
    event_name: Option<String>,
    hostname: Option<String>,
    http_path: Option<String>,
    severity: Option<String>,
    http_url: Option<String>,
}
pub async fn list_events_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    Query(query): Query<Params>,
) -> impl IntoResponse {
    let app = appcontext.lock().await;
    let eventsdb = app.eventsdb.as_ref().unwrap();
    let _db = eventsdb.lock().await;

    let mut qb = QueryBuilder::new("SELECT * FROM evt_events WHERE 1=1 ");
    if let Some(name) = query.event_name.filter(|s| !s.trim().is_empty()) {
        qb.push(" AND event_name LIKE ")
            .push_bind(format!("%{name}%"));
    }
    if let Some(hostname) = query.hostname.filter(|s| !s.trim().is_empty()) {
        qb.push(" AND hostname LIKE ").push_bind(format!("%{hostname}%"));
    }
    if let Some(path) = query.http_path.filter(|s| !s.trim().is_empty()) {
        qb.push(" AND http_path LIKE ").push_bind(format!("%{path}%"));
    }
    if let Some(severity) = query.severity {
        //qb.push(" AND severity = ").push_bind(severity);
    }
    if let Some(url) = query.http_url {
        //qb.push(" AND http_url LIKE ").push_bind(format!("%{url}%"));
    }

    qb.push(" ORDER BY id DESC");
    qb.push(" LIMIT ").push_bind(query.per_page.unwrap_or(15));
    qb.push(" OFFSET ")
        .push_bind(query.page.unwrap_or(0) * query.per_page.unwrap_or(15));

    let mut rows = qb.build().fetch(_db.pool.as_ref().unwrap());

    let mut results = Vec::new();
    while let Some(Ok(row)) = rows.next().await {
        println!("parsing item");
        let e = LogEvent {
            timestamp: row.get("timestamp"),
            event_name: row.get("event_name"),
            ui: row.get("ui"),
            service_name: row.get("service_name"),
            severity: row.get("severity"),
            message: row.get("message"),
            error: Some(ErrorInfo {
                error_type: row.get("error_type"),
                error_message: row.get("error_message"),
                stack_trace: row.get("stack_trace"),
            }),
            app: Some(AppInfo {
                instance_id: row.get("app_instance_id"),
                build_commit: row.get("build_commit"),
                build_id: row.get("build_id"),
                region: row.get("app_region"),
            }),
            service: Some(ServiceInfo {
                version: row.get("service_version"),
                environment: row.get("environment"),
            }),
            host: Some(HostInfo {
                hostname: row.get("hostname"),
                host_ip: row.get("host_ip"),
                region: row.get("host_region"),
                provider: row.get("host_provider"),
            }),
            tracing: Some(TracingInfo {
                trace_id: row.get("trace_id"),
                span_id: row.get("span_id"),
                transaction_id: row.get("transaction_id"),
            }),
            user: Some(UserInfo {
                id: row.get("user_id"),
                name: row.get("user_name"),
                email: row.get("user_email"),
                session_id: row.get("session_id"),
            }),
            http: Some(HttpInfo {
                method: row.get("http_method"),
                path: row.get("http_path"),
                url: row.get("http_url"),
                origin: row.get("http_origin"),
                status: row.get("http_status"),
                client_ip: row.get("client_ip"),
                user_agent: row.get("user_agent"),
            }),
            request: Some(RequestInfo {
                request_id: row.get("request_id"),
                referrer: row.get("referrer"),
                protocol: row.get("protocol"),
                response_size_bytes: row.get("response_size_bytes"),
            }),
            tags: Some(serde_json::from_str(row.get("tags")).unwrap_or_default()),
            labels: Some(serde_json::from_str(row.get("labels")).unwrap_or_default()),
            meta: Some(serde_json::from_str(row.get("meta")).unwrap_or_default()),
        };
        results.push(e);
    }

    AppResponse {
        status: 200,
        message: "Ok".to_owned(),
        data: Some(results),
    }
}
