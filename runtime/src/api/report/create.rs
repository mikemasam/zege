#![allow(dead_code)]
use crate::api::report::list::ZegeReport;
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::utils::http::{AppResponse, AppResult};
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
    Extension(ctx): Extension<Arc<AppContext>>,
    axum::Json(item): axum::extract::Json<ReportCreate>,
) -> AppResult<ZegeReport> {
    let report = match ctx.storage.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => sqlite_write(pool, item).await?,
        DatabasePool::Postgres(pool) => pgsql_write(pool, item).await?,
    };
    AppResponse::created(Some(report), None)
}

async fn sqlite_write(pool: &SqlitePool, item: ReportCreate) -> Result<ZegeReport, sqlx::Error> {
    todo!("report_sqlite_write")
}
async fn pgsql_write(pool: &PgPool, item: ReportCreate) -> Result<ZegeReport, sqlx::Error> {
    let id = item.id.as_ref();
    let sql = match id {
        Some(_) => {
            " update reports set report_name = ?, report_type = ?, report_sql = ?, updated_at = ? where id = ?  RETURNING *"
        }
        None => {
            " insert into reports (report_name, report_type, report_sql, created_at, updated_at) VALUES (?, ?, ?, ?, ?) RETURNING *"
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
