use crate::{
    api::report::list::ZegeReport, api_ensure, ctx::{appcontext::AppContext, dbmanager::DatabasePool}, utils::{
        dbutil::rows_to_json_vec,
        http::{AppResponse, AppResult},
    }
};
use axum::extract::{Extension, Path};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
pub struct ReadOutput {
    data: Option<Vec<Value>>,
    report: ZegeReport,
}
pub async fn report_read_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Path(id): Path<i32>,
) -> AppResult<ReadOutput> {

    let sql = "SELECT * FROM zg_reports where id = ?";
    let report = match ctx.storage.pool.as_ref().unwrap() {
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

    api_ensure!(report.is_ok(), "Report not found");
    let rows = match ctx.storage.pool.as_ref().unwrap() {
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
