#![allow(dead_code)]
use crate::{ctx::dbmanager::DbManager, event::event::LogEvent};
use anyhow::Result;
use std::sync::{Arc, mpsc::Receiver};
use tokio::sync::Mutex;

pub async fn event_write_worker(receiver: Receiver<LogEvent>) {
    let _db = DbManager::new("data/events.db", None).await;
    if _db.is_err() {
        panic!(
            "Failed to open events db with error {:?}",
            _db.err().unwrap()
        );
    };

    let db = Arc::new(Mutex::new(_db.unwrap()));
    for event in receiver {
        write_event(db.clone(), event).await;
    }
}

async fn write_event(eventsdb: Arc<Mutex<DbManager>>, e: LogEvent) -> Result<u64> {
    println!(
        "> {} - {}:{} - {}",
        e.timestamp,
        e.service_name,
        e.event_name,
        e.message.clone().unwrap_or("".to_owned()).as_str()
    );
    let query = String::from(
        "INSERT INTO evt_events (
    timestamp, severity, message,

    error_type, error_message, stack_trace,

    app_instance_id, build_commit, build_id, app_region,

    service_name, service_version, environment,

    hostname , host_ip , host_region , host_provider ,

    trace_id, span_id, transaction_id,

    user_id, user_name, user_email, session_id,

    http_method, http_path, http_status, client_ip, user_agent,

    request_id, referrer, protocol, response_size_bytes,

    tags,  labels, meta, event_name
    ) VALUES 
(
$1,$2,$3,
$4,$5,$6,
$7,$8,$9,$10,
$11,$12,$13,
$14,$15,$16,$17,
$18,$19,$20,
$21,$22,$23,$24,
$25,$26,$27,$28,$29,
$30,$31,$32,$33,
$34,$35,$36, $37
)
",
    );
    let db = eventsdb.as_ref().lock().await;
    let res = sqlx::query(query.as_str())
        .bind(e.timestamp)
        .bind(e.severity)
        .bind(e.message)
        .bind(e.error.as_ref().map(|v| &v.error_type))
        .bind(e.error.as_ref().map(|v| &v.error_message))
        .bind(e.error.as_ref().map(|v| &v.stack_trace))
        .bind(e.app.as_ref().map(|v| &v.instance_id))
        .bind(e.app.as_ref().map(|v| &v.build_commit))
        .bind(e.app.as_ref().map(|v| &v.build_id))
        .bind(e.app.as_ref().map(|v| &v.region))
        .bind(e.service_name)
        .bind(e.service.as_ref().map(|v| &v.version))
        .bind(e.service.as_ref().map(|v| &v.environment))
        //hostname , host_ip , host_region , host_provider ,
        .bind(e.host.as_ref().map(|v| &v.hostname))
        .bind(e.host.as_ref().map(|v| &v.host_ip))
        .bind(e.host.as_ref().map(|v| &v.region))
        .bind(e.host.as_ref().map(|v| &v.provider))
        .bind(e.tracing.as_ref().map(|v| &v.trace_id))
        .bind(e.tracing.as_ref().map(|v| &v.span_id))
        .bind(e.tracing.as_ref().map(|v| &v.transaction_id))
        .bind(e.user.as_ref().map(|v| &v.id))
        .bind(e.user.as_ref().map(|v| &v.name))
        .bind(e.user.as_ref().map(|v| &v.email))
        .bind(e.user.as_ref().map(|v| &v.session_id))
        .bind(e.http.as_ref().map(|v| &v.method))
        .bind(e.http.as_ref().map(|v| &v.path))
        .bind(e.http.as_ref().map(|v| &v.status))
        .bind(e.http.as_ref().map(|v| &v.client_ip))
        .bind(e.http.as_ref().map(|v| &v.user_agent))
        .bind(e.request.as_ref().map(|v| &v.request_id))
        .bind(e.request.as_ref().map(|v| &v.referrer))
        .bind(e.request.as_ref().map(|v| &v.protocol))
        .bind(e.request.as_ref().map(|v| &v.response_size_bytes))
        .bind(e.tags.map(|v| serde_json::to_value(v).ok()))
        .bind(e.labels.map(|v| serde_json::to_value(v).ok()))
        .bind(e.meta.map(|v| serde_json::to_value(v).ok()))
        .bind(e.event_name)
        .execute(db.pool.as_ref().unwrap())
        .await?;
    Ok(res.rows_affected())
}
