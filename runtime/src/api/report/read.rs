use crate::{
    api::report::list::ZegeReport,
    api_ensure,
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    lib::auth::user::papers::UserPaper,
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
    let sql = "SELECT * FROM reports where id = $1 and organization_id = $2";
    let report = match ctx.storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            sqlx::query_as::<_, ZegeReport>(sql)
                .bind(id)
                .bind(organization_id)
                .fetch_one(pool)
                .await
        }
        _ => todo!("report_read_route"),
    };

    api_ensure!(report.is_ok(), "Report not found");

    let rows: Result<std::vec::Vec<serde_json::Value>> = match ctx.storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let mut tx = pool.begin().await?;
            sqlx::query(format!("SET LOCAL app.organization_id = {organization_id}").as_str())
                .execute(tx.as_mut())
                .await?;
            sqlx::query("SET LOCAL ROLE zege_events_read_user")
                .execute(tx.as_mut())
                .await?;
            let res = sqlx::query("SHOW app.organization_id")
                .fetch_one(tx.as_mut())
                .await?;
            println!("After SET: {:?}", res.get_json_value(0));

            let rows = sqlx::query(report.as_ref().unwrap().report_sql.as_str())
                .fetch(tx.as_mut())
                .json(100)
                .await?;

            tx.commit().await?;
            Ok(rows)
        }
        _ => todo!("report_read_route"),
    };

    let output = ReadOutput {
        data: rows.ok(),
        report: report.unwrap(),
    };
    AppResponse::ok(Some(output), None)
}
