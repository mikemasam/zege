use crate::{ctx::appcontext::AppContext, utils::http::AppResponse};
use axum::{extract::Extension, response::IntoResponse};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ZegeReport {
    pub id: i64,
    pub report_name: String,
    pub report_type: String,
    pub report_sql: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn report_index_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
) -> impl IntoResponse {
    let app = appcontext.lock().await;
    let configdb = app.configdb.as_ref().unwrap();
    let db = configdb.lock().await;
    let reports = sqlx::query_as::<_, ZegeReport>("SELECT * FROM zg_reports ORDER BY id DESC")
        .fetch_all(db.pool.as_ref().unwrap())
        .await;
    AppResponse::ok(reports.ok(), None)
}
