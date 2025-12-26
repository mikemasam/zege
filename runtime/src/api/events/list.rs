#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::dto::logevent::{LogEvent, ZegeEventRow};
use crate::utils::http::{AppResponse, AppResult};
use axum::{Extension, extract::Query};
use futures::StreamExt;
use serde::Deserialize;
use sqlx::{Pool, Postgres, QueryBuilder, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct QueryParams {
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
    Extension(ctx): Extension<Arc<AppContext>>,
    Query(query_params): Query<QueryParams>,
) -> AppResult<Vec<LogEvent>> {
    let rows = match ctx.storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => fetch_postgres(pool, query_params).await,
        DatabasePool::Sqlite(pool) => fetch_sqlite(pool, query_params).await,
    };
    AppResponse::ok(Some(rows), None)
}

async fn fetch_postgres(pool: &Pool<Postgres>, query_params: QueryParams) -> Vec<LogEvent> {
    let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM evt_events WHERE 1=1");

    if let Some(name) = query_params.event_name.filter(|s| !s.trim().is_empty()) {
        qb.push(" AND event_name LIKE ")
            .push_bind(format!("%{name}%"));
    }
    if let Some(hostname) = query_params.hostname.filter(|s| !s.trim().is_empty()) {
        qb.push(" AND hostname LIKE ")
            .push_bind(format!("%{hostname}%"));
    }
    if let Some(path) = query_params.http_path.filter(|s| !s.trim().is_empty()) {
        qb.push(" AND http_path LIKE ")
            .push_bind(format!("%{path}%"));
    }
    //qb.push(" AND ui = '019b03b0a51670a0b6ae75ed7a612440'");

    qb.push(" ORDER BY id DESC");
    qb.push(" LIMIT ")
        .push_bind(Into::<i64>::into(query_params.per_page.unwrap_or(15)));
    qb.push(" OFFSET ").push_bind(Into::<i64>::into(
        query_params.page.unwrap_or(0) * query_params.per_page.unwrap_or(15),
    ));

    // Build a typed query
    let query = qb.build_query_as::<ZegeEventRow>();

    // Execute
    let mut rows = query.fetch(pool);
    let mut results = Vec::new();

    while let Some(row) = rows.next().await {
        match row {
            Ok(r) => {
                results.push(r.to_event());
            }
            Err(e) => {
                eprintln!("error fetching row: {:?}", e);
                break;
            }
        }
    }

    results
}

async fn fetch_sqlite(pool: &Pool<Sqlite>, query_params: QueryParams) -> Vec<LogEvent> {
    todo!("events_list_fetch_sqlite")
}
