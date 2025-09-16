#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::event::event::LogEvent;
use crate::http::AppResponse;
use axum::response::IntoResponse;
use axum::{Extension, extract::Query};
use futures::StreamExt;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use sqlx::{Column, Row, SqlitePool};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct Params {
    search: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
}
pub async fn list_events_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    Query(query): Query<Params>,
) -> impl IntoResponse {
    let app = appcontext.lock().await;
    let eventsdb = app.eventsdb.as_ref().unwrap();
    let _db = eventsdb.lock().await;
    let offset = query.page.unwrap_or(0) * query.per_page.unwrap_or(15);
    let mut rows = sqlx::query("SELECT * FROM evt_events ORDER BY id desc limit $1 OFFSET $2")
        .bind(query.per_page.unwrap_or(15))
        .bind(offset)
        .fetch(_db.pool.as_ref().unwrap());

    let mut result = Vec::new();
    while let Some(Ok(row)) = rows.next().await {
        let mut map = Map::new();
        for column in row.columns() {
            let name = column.name().to_string();
            let value: Value = row
                .try_get::<Value, &str>(name.as_str())
                .or_else(|_| row.try_get::<String, _>(name.as_str()).map(Value::from))
                .or_else(|_| row.try_get::<f64, _>(name.as_str()).map(Value::from))
                .or_else(|_| row.try_get::<i64, _>(name.as_str()).map(Value::from))
                .or_else(|_| row.try_get::<bool, _>(name.as_str()).map(Value::from))
                .unwrap_or(json!(null));
            map.insert(name.to_string(), value);
        }
        result.push(Value::Object(map));
    }

    AppResponse {
        status: 200,
        message: String::new(),
        data: Some(result),
    }
}
