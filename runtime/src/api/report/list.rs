use crate::{
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
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
) -> AppResult<Vec<ZegeReport>> {
    let app = appcontext.lock().await;
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    let sql = "SELECT * FROM zg_reports ORDER BY id DESC";
    let reports = match db.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => sqlx::query_as::<_, ZegeReport>(sql).fetch_all(pool).await,
        DatabasePool::Postgres(pool) => sqlx::query_as::<_, ZegeReport>(sql).fetch_all(pool).await,
    };
    AppResponse::ok(reports.ok(), None)
}
