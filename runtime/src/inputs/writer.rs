use crate::{
    ctx::{
        appcontext::DbStorage,
        dbmanager::{DatabasePool, DbManagerConnectOptions, DbPoolManager},
    },
    dto::logevent::LogEvent,
    lib::{
        buckets::Bucket,
        events::input::{LogEventChannelMessage, LogEventInput},
    },
    utils::{appconfig::applogger, security::Security},
};
use chrono::{Local, SecondsFormat};
use serde_json::Value;
use sqlx::{Error, PgPool, QueryBuilder, SqlitePool};
use std::{
    collections::HashMap,
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
                match event.inject(db.clone()).await {
                    Ok(_) => {
                        println!("e {}", serde_json::to_string_pretty(&event).unwrap());
                        events_batch.push(*event)
                    }
                    Err(e) => applogger::error(format!("{:?}", e)),
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
        applogger::log("# write size: empty, wrote: 0, time: 0".to_string());
        return;
    }
    let start_time = Instant::now();
    let written_events_count = match write_events(eventsdb, events_batch).await {
        Ok(t) => t,
        Err(err) => {
            applogger::error(format!("##### WRITE ERROR: {err}"));
            0
        }
    };
    let elapsed_time = start_time.elapsed();
    let size = events_batch.len();
    if written_events_count > 0 {
        events_batch.clear();
    }
    applogger::log(format!(
        "# write size: {size}, wrote: {written_events_count}, time: {elapsed_time:?}"
    ));
}
async fn write_events(
    eventsdb: Arc<DbPoolManager>,
    events: &Vec<LogEventInput>,
) -> Result<u64, Error> {
    for e in events {
        applogger::debug(format!(
            "> {} - {:?}:{} - {}",
            e.timestamp,
            e.service,
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
    timestamp, message,  version, service,
    host, data, event_name, event_type, 
    event_bucket_id, event_organization_id, event_ui, event_created_at 
    ) ";
    let mut query = QueryBuilder::<sqlx::Postgres>::new(INSERT_SQL);

    query.push_values(events, |mut b, e| {
        b.push_bind(e.timestamp)
            .push_bind(e.message.clone())
            .push_bind(e.version.clone())
            .push_bind(e.service.clone())
            .push_bind(e.host.clone())
            .push_bind(e.data.clone().map(|v| serde_json::to_value(v).ok()))
            .push_bind(e.event_name.clone())
            .push_bind(e.event_type.clone())
            .push_bind(e.event_bucket_id)
            .push_bind(e.event_organization_id)
            .push_bind(Uuid::now_v7().simple().to_string())
            .push_bind(Local::now());
    });
    let res = query.build().execute(pool).await?;
    Ok(res.rows_affected())
}
