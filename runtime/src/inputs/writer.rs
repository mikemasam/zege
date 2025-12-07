#![allow(dead_code, unused_imports, unused_variables)]
use crate::{
    ctx::{
        appcontext::AppEnv,
        dbmanager::{DatabasePool, DbManager, DbManagerConnectOptions},
    },
    dto::logevent::{LogEvent, LogEventChannelMessage},
};
use chrono::{Local, SecondsFormat};
use sqlx::{Error, PgPool, QueryBuilder, SqlitePool};
use std::{
    ops::DerefMut,
    sync::{
        Arc,
        mpsc::{Receiver, RecvError, RecvTimeoutError},
    },
    time::Duration,
};
use tokio::runtime::{Handle, Runtime};
use tokio::{sync::Mutex, time::Instant};
use uuid::Uuid;

pub fn start_events_writer_thread(
    receiver: Receiver<LogEventChannelMessage>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async { event_write_worker(receiver).await });
    })
}

async fn event_write_worker(receiver: Receiver<LogEventChannelMessage>) {
    let _db = DbManager::connect(DbManagerConnectOptions {
        migrate: false,
        backup: false,
    })
    .await;
    if _db.is_err() {
        panic!(
            "Failed to open events db with error {:?}",
            _db.err().unwrap()
        );
    };

    let mut events_batch = vec![];
    let db = Arc::new(Mutex::new(_db.unwrap()));
    loop {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(LogEventChannelMessage::Data(mut event)) => {
                event.ui = Some(Uuid::now_v7().simple().to_string());
                events_batch.push(*event);
                if events_batch.len() >= 100 {
                    time_write_events(db.clone(), &mut events_batch).await;
                }
            }
            Ok(LogEventChannelMessage::Shutdown) => {
                time_write_events(db.clone(), &mut events_batch).await;
                break;
            }
            Err(RecvTimeoutError::Timeout) => {
                if !events_batch.is_empty() {
                    time_write_events(db.clone(), &mut events_batch).await;
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                time_write_events(db.clone(), &mut events_batch).await;
                break;
            }
        }
        //println!("pending size {} ", events_batch.len());
    }
}

async fn time_write_events(eventsdb: Arc<Mutex<DbManager>>, events_batch: &mut Vec<LogEvent>) {
    if events_batch.is_empty() {
        AppEnv::log("# EventWrite size: empty, wrote: 0, time: 0".to_string());
        return;
    }
    let start_time = Instant::now();
    let written_events_count = match write_events(eventsdb, events_batch).await {
        Ok(t) => t,
        Err(err) => {
            AppEnv::error(format!("##### WRITE ERROR: {err}"));
            0
        }
    };
    let elapsed_time = start_time.elapsed();
    let size = events_batch.len();
    if written_events_count > 0 {
        events_batch.clear();
    }
    AppEnv::log(format!(
        "# EventWrite size: {size}, wrote: {written_events_count}, time: {elapsed_time:?}"
    ));
}
async fn write_events(
    eventsdb: Arc<Mutex<DbManager>>,
    events: &Vec<LogEvent>,
) -> Result<u64, Error> {
    for e in events {
        AppEnv::debug(format!(
            "> {} - {}:{} - {}",
            e.timestamp,
            e.service_name,
            e.event_name,
            e.message.clone().unwrap_or("".to_owned()).as_str()
        ));
    }
    let db = eventsdb.as_ref().lock().await;
    match db.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => sqlite_write_events(pool, events).await,
        DatabasePool::Postgres(pool) => pgsql_write_events(pool, events).await,
    }
}
static INSERT_SQL: &str = "INSERT INTO evt_events (
    timestamp, severity, message,
    error_type, error_message, stack_trace,
    app_instance_id, build_commit, build_id, app_region,
    service_name, service_version, environment,
    hostname , host_ip , host_region , host_provider ,
    trace_id, span_id, transaction_id,
    user_id, user_name, user_email, session_id,
    http_method, http_path, http_status, client_ip, user_agent,
    request_id, referrer, protocol, response_size_bytes,
    tags,  labels, data, event_name, event_type,
    http_url, http_origin, http_headers, ui, _time
    )";
async fn pgsql_write_events(pool: &PgPool, events: &Vec<LogEvent>) -> Result<u64, Error> {
    let mut query = QueryBuilder::<sqlx::Postgres>::new(INSERT_SQL);

    query.push_values(events, |mut b, e| {
        b.push_bind(e.timestamp)
            .push_bind(e.severity.clone())
            .push_bind(e.message.clone())
            .push_bind(e.error.as_ref().map(|v| &v.error_type))
            .push_bind(e.error.as_ref().map(|v| &v.error_message))
            .push_bind(e.error.as_ref().map(|v| &v.stack_trace))
            .push_bind(e.app.as_ref().map(|v| &v.instance_id))
            .push_bind(e.app.as_ref().map(|v| &v.build_commit))
            .push_bind(e.app.as_ref().map(|v| &v.build_id))
            .push_bind(e.app.as_ref().map(|v| &v.region))
            .push_bind(e.service_name.clone())
            .push_bind(e.service.as_ref().map(|v| &v.version))
            .push_bind(e.service.as_ref().map(|v| &v.environment))
            .push_bind(e.host.as_ref().map(|v| &v.hostname))
            .push_bind(e.host.as_ref().map(|v| &v.host_ip))
            .push_bind(e.host.as_ref().map(|v| &v.region))
            .push_bind(e.host.as_ref().map(|v| &v.provider))
            .push_bind(e.tracing.as_ref().map(|v| &v.trace_id))
            .push_bind(e.tracing.as_ref().map(|v| &v.span_id))
            .push_bind(e.tracing.as_ref().map(|v| &v.transaction_id))
            .push_bind(e.user.as_ref().map(|v| &v.id))
            .push_bind(e.user.as_ref().map(|v| &v.name))
            .push_bind(e.user.as_ref().map(|v| &v.email))
            .push_bind(e.user.as_ref().map(|v| &v.session_id))
            .push_bind(e.http.as_ref().map(|v| &v.method))
            .push_bind(e.http.as_ref().map(|v| &v.path))
            .push_bind(e.http.as_ref().map(|v| &v.status))
            .push_bind(e.http.as_ref().map(|v| &v.client_ip))
            .push_bind(e.http.as_ref().map(|v| &v.user_agent))
            .push_bind(e.request.as_ref().map(|v| &v.request_id))
            .push_bind(e.request.as_ref().map(|v| &v.referrer))
            .push_bind(e.request.as_ref().map(|v| &v.protocol))
            .push_bind(e.request.as_ref().map(|v| &v.response_size_bytes))
            .push_bind(e.tags.clone().map(|v| serde_json::to_value(v).ok()))
            .push_bind(e.labels.clone().map(|v| serde_json::to_value(v).ok()))
            .push_bind(e.data.clone().map(|v| serde_json::to_value(v).ok()))
            .push_bind(e.event_name.clone())
            .push_bind(e.event_type.clone())
            .push_bind(e.http.as_ref().map(|v| &v.url))
            .push_bind(e.http.as_ref().map(|v| &v.origin))
            .push_bind(
                e.http
                    .as_ref()
                    .map(|v| serde_json::to_value(v.headers.clone()).ok()),
            )
            .push_bind(e.ui.clone())
            .push_bind(Local::now());
    });
    let res = query.build().execute(pool).await?;
    Ok(res.rows_affected())
}

async fn sqlite_write_events(pool: &SqlitePool, events: &Vec<LogEvent>) -> Result<u64, Error> {
    let mut query = QueryBuilder::<sqlx::Sqlite>::new(INSERT_SQL);
    query.push_values(events, |mut b, e| {
        b.push_bind(e.timestamp)
            .push_bind(e.severity.clone())
            .push_bind(e.message.clone())
            .push_bind(e.error.as_ref().map(|v| &v.error_type))
            .push_bind(e.error.as_ref().map(|v| &v.error_message))
            .push_bind(e.error.as_ref().map(|v| &v.stack_trace))
            .push_bind(e.app.as_ref().map(|v| &v.instance_id))
            .push_bind(e.app.as_ref().map(|v| &v.build_commit))
            .push_bind(e.app.as_ref().map(|v| &v.build_id))
            .push_bind(e.app.as_ref().map(|v| &v.region))
            .push_bind(e.service_name.clone())
            .push_bind(e.service.as_ref().map(|v| &v.version))
            .push_bind(e.service.as_ref().map(|v| &v.environment))
            .push_bind(e.host.as_ref().map(|v| &v.hostname))
            .push_bind(e.host.as_ref().map(|v| &v.host_ip))
            .push_bind(e.host.as_ref().map(|v| &v.region))
            .push_bind(e.host.as_ref().map(|v| &v.provider))
            .push_bind(e.tracing.as_ref().map(|v| &v.trace_id))
            .push_bind(e.tracing.as_ref().map(|v| &v.span_id))
            .push_bind(e.tracing.as_ref().map(|v| &v.transaction_id))
            .push_bind(e.user.as_ref().map(|v| &v.id))
            .push_bind(e.user.as_ref().map(|v| &v.name))
            .push_bind(e.user.as_ref().map(|v| &v.email))
            .push_bind(e.user.as_ref().map(|v| &v.session_id))
            .push_bind(e.http.as_ref().map(|v| &v.method))
            .push_bind(e.http.as_ref().map(|v| &v.path))
            .push_bind(e.http.as_ref().map(|v| &v.status))
            .push_bind(e.http.as_ref().map(|v| &v.client_ip))
            .push_bind(e.http.as_ref().map(|v| &v.user_agent))
            .push_bind(e.request.as_ref().map(|v| &v.request_id))
            .push_bind(e.request.as_ref().map(|v| &v.referrer))
            .push_bind(e.request.as_ref().map(|v| &v.protocol))
            .push_bind(e.request.as_ref().map(|v| &v.response_size_bytes))
            .push_bind(e.tags.clone().map(|v| serde_json::to_value(v).ok()))
            .push_bind(e.labels.clone().map(|v| serde_json::to_value(v).ok()))
            .push_bind(e.data.clone().map(|v| serde_json::to_value(v).ok()))
            .push_bind(e.event_name.clone())
            .push_bind(e.event_type.clone())
            .push_bind(e.http.as_ref().map(|v| &v.url))
            .push_bind(e.http.as_ref().map(|v| &v.origin))
            .push_bind(
                e.http
                    .as_ref()
                    .map(|v| serde_json::to_value(v.headers.clone()).ok()),
            )
            .push_bind(e.ui.clone())
            .push_bind(Local::now());
    });
    let res = query.build().execute(pool).await?;
    Ok(res.rows_affected())
}
