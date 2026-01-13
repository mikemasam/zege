use crate::{
    api::report::list::ZegeReport,
    api_ensure,
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    lib::{auth::user::papers::UserPaper, data::reader::DataReader},
    utils::{
        dbutil::{JsonRow, StreamJsonExt},
        http::{AppResponse, AppResult},
    },
};
use anyhow::Result;
use axum::extract::{Extension, Path};
use serde::Serialize;
use serde_json::Value;
use sqlx::Acquire;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
pub struct ReadOutput {
    data: Option<Vec<Value>>,
    report: ZegeReport,
}
pub async fn report_read_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
    Path(id): Path<i32>,
) -> AppResult<ReadOutput> {
    let organization_id = paper.organization.map(|o| o.id).unwrap();
    let report = match ctx.storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            sqlx::query_as::<_, ZegeReport>(
                "SELECT * FROM reports where id = $1 and organization_id = $2",
            )
            .bind(id)
            .bind(organization_id)
            .fetch_one(pool)
            .await
        }
        _ => todo!("report_read_route"),
    };

    api_ensure!(report.is_ok(), "Report not found");

    let rows = DataReader::read(
        ctx.storage.clone(),
        organization_id,
        report.as_ref().unwrap().report_sql.clone(),
    )
    .await;

    let output = ReadOutput {
        data: rows.ok(),
        report: report.unwrap(),
    };
    AppResponse::ok(Some(output), None)
}
