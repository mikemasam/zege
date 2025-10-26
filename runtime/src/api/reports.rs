use crate::{ctx::appcontext::AppContext, utils::http::AppResponse};
use axum::{Json, extract::Extension, response::IntoResponse};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Report {
    id: i64,
    report_name: String,
    report_type: String,
    report_sql: String,
    created_at: String,
    updated_at: String,
}

pub async fn report_index_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
) -> impl IntoResponse {
    let app = appcontext.lock().await;
    let configdb = app.configdb.as_ref().unwrap();
    let db = configdb.lock().await;
    let reports = sqlx::query_as::<_, Report>("SELECT * FROM zg_reports ORDER BY id DESC")
        .fetch_all(db.pool.as_ref().unwrap())
        .await;
    Json(AppResponse {
        status: 200,
        message: String::new(),
        data: reports.ok(),
    })
}
