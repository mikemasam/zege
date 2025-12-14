#![allow(dead_code)]
use crate::api::report::list::ZegeReport;
use crate::ctx::dbmanager::DatabasePool;
use crate::utils::http::{AppResponse, AppResult};
use crate::{ctx::appcontext::AppContext};
use axum::Extension;
use chrono::Local;
use serde::Deserialize;
use sqlx::{PgPool, SqlitePool};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct ReportCreate {
    id: Option<i32>,
    report_name: String,
    report_sql: String,
    report_type: String,
}

pub async fn report_create_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    axum::Json(item): axum::extract::Json<ReportCreate>,
) -> AppResult<ZegeReport> {
    let app = appcontext.lock().await;
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;

    let report = match db.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => sqlite_write(pool, item).await?,
        DatabasePool::Postgres(pool) => pgsql_write(pool, item).await?,
    };
    AppResponse::created(Some(report), None)
}

async fn sqlite_write(pool: &SqlitePool, item: ReportCreate) -> Result<ZegeReport, sqlx::Error> {
    let id = item.id.as_ref();
    let sql = match id {
        Some(_) => {
            " update zg_reports set report_name = ?, report_type = ?, report_sql = ?, updated_at = ? where id = ?"
        }
        None => {
            " insert into zg_reports (report_name, report_type, report_sql, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
        }
    };
    let mut q = sqlx::query_as::<_, ZegeReport>(sql)
        .bind(&item.report_name)
        .bind(item.report_type)
        .bind(item.report_sql)
        .bind(Local::now().to_rfc3339());
    if id.is_some() {
        q = q.bind(item.id);
    } else {
        q = q.bind(Local::now().to_rfc3339());
    }
    q.fetch_one(pool).await
}
async fn pgsql_write(pool: &PgPool, item: ReportCreate) -> Result<ZegeReport, sqlx::Error> {
    let id = item.id.as_ref();
    let sql = match id {
        Some(_) => {
            " update zg_reports set report_name = ?, report_type = ?, report_sql = ?, updated_at = ? where id = ?  RETURNING *"
        }
        None => {
            " insert into zg_reports (report_name, report_type, report_sql, created_at, updated_at) VALUES (?, ?, ?, ?, ?) RETURNING *"
        }
    };
    let mut q = sqlx::query_as::<_, ZegeReport>(sql)
        .bind(&item.report_name)
        .bind(item.report_type)
        .bind(item.report_sql)
        .bind(Local::now());
    if id.is_some() {
        q = q.bind(item.id);
    } else {
        q = q.bind(Local::now());
    }
    q.fetch_one(pool).await
}
