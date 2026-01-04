use redis::{AsyncCommands, Client};
use serde_json::Value;
use std::{env, sync::Arc};
use tokio::time::{Duration, sleep};

use crate::{ctx::appcontext::AppContext, dto::logevent::LogEvent, lib::events::input::{LogEventChannelMessage, LogEventInput}, utils::appenv::AppLogger};

pub async fn start_redis_reader(ctx: Arc<AppContext>) {
    let conns = env::var("REDIS_SERVERS").unwrap();
    let conns_list: Vec<String> = conns.split(',').map(|s| s.to_string()).collect();

    for url in conns_list {
        connect_to_server(ctx.clone(), url);
    }
}
fn connect_to_server(ctx: Arc<AppContext>, url: String) {
    tokio::task::spawn(async move {
        loop {
            AppLogger::log(format!("Redis URL: {url}"));
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
    AppLogger::log(format!("Connecting to Redis -> {}", url));

    let client = Client::open(url)?;
    let mut conn = client.get_multiplexed_async_connection().await?;

    AppLogger::log(format!("Redis connected -> {}", url));

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
                        eprintln!("event_writer error: {}", e);
                    }
                }
                Err(e) => eprintln!("JSON parse error: {}", e),
            },
            Err(e) => {
                eprintln!("BLPOP error on {} -> {}", url, e);
                return Err(e); // triggers reconnect loop
            }
        }
    }
}

/// Stub replacement for event$writer
async fn event_writer(ctx: Arc<AppContext>, event: serde_json::Value) -> Result<(), String> {
    //println!("{}", serde_json::to_string_pretty(&event).unwrap());
    let parser: Result<LogEventInput, serde_json::Error> = serde_json::from_value(event);
    if parser.is_err() {
        eprintln!("> Redis: Failed to process event {:?}", parser.err());
        return Ok(());
    }
    let wr = ctx
        .event_writer
        .send(LogEventChannelMessage::Data(Box::new(parser.unwrap())));
    if wr.is_err() {
        eprintln!("> Redis: Failed to process event {:?}", wr.err());
        return Ok(());
    }
    Ok(())
}
