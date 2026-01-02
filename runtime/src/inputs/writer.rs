use crate::{
    ctx::{
        appcontext::DbStorage,
        dbmanager::{DatabasePool, DbManagerConnectOptions, DbPoolManager},
    },
    dto::logevent::{LogEvent, LogEventChannelMessage, LogEventInput},
    lib::services::Service,
    utils::appenv::AppLogger,
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
    let _db = DbPoolManager::connect(DbManagerConnectOptions {
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
    let db = Arc::new(_db.unwrap());
    loop {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(LogEventChannelMessage::Data(mut event)) => {
                let service = Service::find_by_apikey(
                    db.clone(),
                    event.as_ref().apikey_value.as_ref().unwrap().to_string(),
                )
                .await;
                if let Ok(s) = service {
                    event.event_service_name = Some(s.name);
                    event.event_service_id = Some(s.id);
                    event.event_organization_id = Some(s.organization_id);
                    events_batch.push(*event);
                } else {
                    AppLogger::error(format!(
                        "event api key not found {} for event {}",
                        event.apikey_value.unwrap_or_default(),
                        event.event_name
                    ));
                }
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

async fn time_write_events(eventsdb: Arc<DbPoolManager>, events_batch: &mut Vec<LogEventInput>) {
    if events_batch.is_empty() {
        AppLogger::log("# write size: empty, wrote: 0, time: 0".to_string());
        return;
    }
    let start_time = Instant::now();
    let written_events_count = match write_events(eventsdb, events_batch).await {
        Ok(t) => t,
        Err(err) => {
            AppLogger::error(format!("##### WRITE ERROR: {err}"));
            0
        }
    };
    let elapsed_time = start_time.elapsed();
    let size = events_batch.len();
    if written_events_count > 0 {
        events_batch.clear();
    }
    AppLogger::log(format!(
        "# write size: {size}, wrote: {written_events_count}, time: {elapsed_time:?}"
    ));
}
async fn write_events(
    eventsdb: Arc<DbPoolManager>,
    events: &Vec<LogEventInput>,
) -> Result<u64, Error> {
    for e in events {
        AppLogger::debug(format!(
            "> {} - {:?}:{} - {}",
            e.timestamp,
            e.event_service_name,
            e.event_name,
            e.message.clone().unwrap_or("".to_owned()).as_str()
        ));
    }
    match eventsdb.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => pgsql_write_events(pool, events).await,
        _ => todo!("db driver not implementated"),
    }
}
async fn pgsql_write_events(pool: &PgPool, events: &Vec<LogEventInput>) -> Result<u64, Error> {
    let INSERT_SQL: &str = "INSERT INTO zege_events (
    timestamp, severity, message,
    error_type, error_message, stack_trace,
    app_instance_id, build_commit, build_id, app_region,
    service_version, environment,
    hostname , host_ip , host_region , host_provider ,
    trace_id, span_id, transaction_id,
    user_id, user_name, user_email, session_id,
    http_method, http_path, http_status, client_ip, user_agent,
    request_id, referrer, protocol, response_size_bytes,
    tags,  labels, data, event_name, event_type,
    http_url, http_origin, http_headers, 
    event_service_name, event_service_id, event_organization_id, event_ui, event_created_at
    )";
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
            .push_bind(e.event_service_name.clone())
            .push_bind(e.event_service_id)
            .push_bind(e.event_organization_id)
            .push_bind(Uuid::now_v7().simple().to_string())
            .push_bind(Local::now());
    });
    let res = query.build().execute(pool).await?;
    Ok(res.rows_affected())
}
