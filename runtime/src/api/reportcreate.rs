#![allow(dead_code)]
use crate::utils::http::AppResponse;
use crate::{api::reports::ZegeReport, ctx::appcontext::AppContext};
use axum::Extension;
use axum::response::IntoResponse;
use chrono::Local;
use serde::Deserialize;
use serde_json::Value;
use sqlx::{Execute, QueryBuilder, query};
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
) -> impl IntoResponse {
    let id = item.id.as_ref();
    println!("{:?}", item);
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
        .bind(Local::now().to_rfc3339());
    let app = appcontext.lock().await;
    let configdb = app.storage.as_ref().unwrap();
    let _db = configdb.lock().await;
    if id.is_some() {
        q = q.bind(item.id);
    } else {
        q = q.bind(Local::now().to_rfc3339());
    }
    println!("SQL: {}", q.sql());
    let report = q.fetch_one(_db.pool.as_ref().unwrap()).await;
    AppResponse::created(report.ok(), None)
}
