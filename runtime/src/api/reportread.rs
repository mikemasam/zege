use crate::{
    api::reports::ZegeReport,
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    utils::{dbutil::rows_to_json_vec, http::AppResponse},
};
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
struct ReadOutput {
    data: Option<Vec<Value>>,
    report: ZegeReport,
}
pub async fn report_read_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let app = appcontext.lock().await;
    let db = app.storage.as_ref().unwrap().lock().await;

    let sql = "SELECT * FROM zg_reports where id = ?";
    let report = match db.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => {
            sqlx::query_as::<_, ZegeReport>(sql)
                .bind(id)
                .fetch_one(pool)
                .await
        }
        DatabasePool::Postgres(pool) => {
            sqlx::query_as::<_, ZegeReport>(sql)
                .bind(id)
                .fetch_one(pool)
                .await
        }
    };
    if report.is_err() {
        return AppResponse::error("Report not found", None);
    }
    let rows = match db.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => {
            let out = sqlx::query(report.as_ref().unwrap().report_sql.as_str()).fetch(pool);
            rows_to_json_vec(out).await
        }
        DatabasePool::Postgres(pool) => {
            let out = sqlx::query(report.as_ref().unwrap().report_sql.as_str()).fetch(pool);
            rows_to_json_vec(out).await
        }
    };
    let output = ReadOutput {
        data: rows.ok(),
        report: report.unwrap(),
    };
    AppResponse::ok(Some(output), None)
}
