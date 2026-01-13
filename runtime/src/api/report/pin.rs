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
pub struct PinOutputItem {
    data: Option<Vec<Value>>,
    name: String,
    r#type: String,
}

pub async fn report_pin_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
) -> AppResult<Vec<PinOutputItem>> {
    let organization_id = paper.organization.map(|o| o.id).unwrap();
    let reports = match ctx.storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            sqlx::query_as::<_, ZegeReport>("SELECT * FROM reports where organization_id = $1")
                .bind(organization_id)
                .fetch_all(pool)
                .await?
        }
        _ => todo!("report_read_route"),
    };
    let mut output: Vec<PinOutputItem> = vec![];
    for report in reports {
        let rows = DataReader::read(ctx.storage.clone(), organization_id, report.report_sql).await;
        output.push(PinOutputItem {
            data: rows.ok(),
            name: report.report_name,
            r#type: report.report_type,
        });
    }
    AppResponse::ok(Some(output), None)
}
