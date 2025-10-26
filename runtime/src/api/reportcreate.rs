#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::utils::http::AppResponse;
use axum::Extension;
use axum::response::IntoResponse;
use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::Value;
use sqlx::QueryBuilder;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct ReportCreate {
    report_name: Option<String>,
    report_sql: Option<String>,
    report_type: Option<String>,
}

pub async fn report_create_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    axum::Json(item): axum::extract::Json<ReportCreate>,
) -> impl IntoResponse {
    let mut query = QueryBuilder::<sqlx::Sqlite>::new(
        " insert into zg_reports (report_name, report_type, report_sql, created_at, updated_at) ",
    );

    query.push_values([item], |mut b, i| {
        b.push_bind(i.report_name)
            .push_bind(i.report_type)
            .push_bind(i.report_sql)
            .push_bind(Local::now())
            .push_bind(Local::now());
    });

    let app = appcontext.lock().await;
    let configdb = app.configdb.as_ref().unwrap();
    let _db = configdb.lock().await;
    let res = query
        .build()
        .execute(_db.pool.as_ref().unwrap())
        .await
        .unwrap();
    AppResponse::<Value> {
        status: 200,
        message: String::new(),
        data: serde_json::to_value(res.last_insert_rowid()).ok(),
    }
}
