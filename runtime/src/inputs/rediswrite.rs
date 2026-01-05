use anyhow::{Result, ensure};
use redis::{AsyncCommands, Client};
use serde_json::Value;
use std::{env, sync::Arc};
use tokio::time::{Duration, sleep};

use crate::{
    appconfig,
    ctx::appcontext::AppContext,
    dto::logevent::LogEvent,
    lib::events::input::{LogEventChannelMessage, LogEventInput},
    utils::appconfig::applogger,
};

pub async fn start_redis_reader(ctx: Arc<AppContext>) {
    let redis_config = &appconfig!().redis;
    if (redis_config.is_none()) {
        return;
    }
    let servers = redis_config
        .as_ref()
        .unwrap()
        .servers
        .clone()
        .unwrap_or_default();
    for url in servers {
        connect_to_server(ctx.clone(), url);
    }
}
fn connect_to_server(ctx: Arc<AppContext>, url: String) {
    tokio::task::spawn(async move {
        loop {
            applogger::log(format!("Redis URL: {url}"));
            match connect_and_listen(ctx.clone(), url.as_str()).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Redis error on {} -> {}", url, e);
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }
    });
}
async fn connect_and_listen(ctx: Arc<AppContext>, url: &str) -> redis::RedisResult<()> {
    applogger::log(format!("Connecting to Redis -> {}", url));

    let client = Client::open(url)?;
    let mut conn = client.get_multiplexed_async_connection().await?;

    applogger::log(format!("Redis connected -> {}", url));

    loop {
        let result: redis::RedisResult<(String, String)> = redis::cmd("BLPOP")
            .arg("zege_events")
            .arg(0)
            .query_async(&mut conn)
            .await;

        match result {
            Ok((_key, payload)) => match serde_json::from_str::<Value>(&payload) {
                Ok(body) => {
                    if let Err(e) = event_writer(ctx.clone(), body).await {
                        applogger::error(format!("event_writer error: {}", e));
                    }
                }
                Err(e) => applogger::error(format!("JSON parse error: {}", e)),
            },
            Err(e) => {
                applogger::error(format!("BLPOP error on: {} -> {}", url, e));
                return Err(e); // triggers reconnect loop
            }
        }
    }
}

/// Stub replacement for event$writer
async fn event_writer(ctx: Arc<AppContext>, eventValue: serde_json::Value) -> Result<()> {
    let parser: Result<LogEventInput, serde_json::Error> =
        serde_json::from_value(eventValue.clone());
    ensure!(
        parser.is_ok(),
        format!("> Redis: Failed to process event {:?}", parser.err())
    );
    let event: LogEventInput = parser.unwrap();

    ensure!(
        event.bucket_key.is_some(),
        format!("event missing bucket_key {}", eventValue)
    );

    let wr = ctx
        .event_writer
        .send(LogEventChannelMessage::Data(Box::new(event)));
    ensure!(
        wr.is_ok(),
        format!("> Redis: Failed to process event {:?}", wr.err())
    );
    Ok(())
}
