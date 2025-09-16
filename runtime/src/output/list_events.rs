#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::event::event::LogEvent;
use crate::http::AppResponse;
use axum::Extension;
use axum::response::IntoResponse;
use futures::StreamExt;
use serde_json::{Map, Value, json};
use sqlx::{Column, Row, SqlitePool};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn list_events_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
) -> impl IntoResponse {
    let app = appcontext.lock().await;
    let eventsdb = app.eventsdb.as_ref().unwrap();
    let _db = eventsdb.lock().await;
    let mut rows =
        sqlx::query("SELECT * FROM evt_events ORDER BY id desc").fetch(_db.pool.as_ref().unwrap());

    let mut result = Vec::new();
    while let Some(Ok(row)) = rows.next().await {
        let mut map = Map::new();
        for column in row.columns() {
            let name = column.name().to_string();
            let value: Value = row
                .try_get::<Value, &str>(name.as_str())
                .or_else(|_| row.try_get::<f64, _>(name.as_str()).map(Value::from))
                .or_else(|_| row.try_get::<i64, _>(name.as_str()).map(Value::from))
                .or_else(|_| row.try_get::<String, _>(name.as_str()).map(Value::from))
                .or_else(|_| row.try_get::<bool, _>(name.as_str()).map(Value::from))
                .unwrap_or(json!(null));
            map.insert(name.to_string(), value);
        }
        result.push(Value::Object(map));
    }

    AppResponse {
        status: 200,
        message: format!(""),
        data: Some(result),
    }
}
