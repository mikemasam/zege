#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::dto::logevent::ZegeEventRow;
use crate::utils::http::AppResponse;
use axum::response::IntoResponse;
use axum::{Extension, extract::Query};
use futures::StreamExt;
use serde::{Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
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
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    Query(query_params): Query<QueryParams>,
) -> impl IntoResponse {
    let app = appcontext.lock().await;
    let storage = app.storage.as_ref().unwrap();
    let _db = storage.lock().await;

    /*
        let filters: &[(&Option<String>, &str)] = &[
            (&query.event_name, "event_name"),
            (&query.hostname, "hostname"),
            (&query.http_path, "http_path"),
        ];
    */

    let mut results = Vec::new();
    let mut rows = match _db.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let qb = sqlx::query_as::<_, ZegeEventRow>("SELECT * FROM evt_events WHERE 1=1 ");
            //apply_where(&mut qb, query_params);
            qb.fetch(pool)
        }
        DatabasePool::Sqlite(pool) => {
            let qb = sqlx::query_as::<_, ZegeEventRow>("SELECT * FROM evt_events WHERE 1=1 ");
            //apply_where(&mut qb, query_params);
            qb.fetch(pool)
        }
    };
    while let Some(Ok(row)) = rows.next().await {
        results.push(row.to_event());
    }

    AppResponse {
        status: 200,
        message: "Ok".to_owned(),
        data: Some(results),
    }
}

fn apply_where<'c, DB>(qb: &mut sqlx::QueryBuilder<'c, DB>, query: QueryParams)
where
    DB: sqlx::Database,
    i64: sqlx::Encode<'c, DB> + sqlx::Type<DB>,
    std::string::String: sqlx::Encode<'c, DB>,
    std::string::String: sqlx::Type<DB>,
{
    if let Some(name) = query.event_name.filter(|s| !s.trim().is_empty()) {
        qb.push(" AND event_name LIKE ")
            .push_bind(format!("%{name}%"));
    }
    if let Some(hostname) = query.hostname.filter(|s| !s.trim().is_empty()) {
        qb.push(" AND hostname LIKE ")
            .push_bind(format!("%{hostname}%"));
    }
    if let Some(path) = query.http_path.filter(|s| !s.trim().is_empty()) {
        qb.push(" AND http_path LIKE ")
            .push_bind(format!("%{path}%"));
    }
    if query.severity.is_some() {
        //qb.push(" AND severity = ").push_bind(severity);
    }
    if query.http_url.is_some() {
        //qb.push(" AND http_url LIKE ").push_bind(format!("%{url}%"));
    }

    qb.push(" ORDER BY id DESC");
    qb.push(" LIMIT ")
        .push_bind(Into::<i64>::into(query.per_page.unwrap_or(15)));
    qb.push(" OFFSET ").push_bind(Into::<i64>::into(
        query.page.unwrap_or(0) * query.per_page.unwrap_or(15),
    ));
}
