use crate::{
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
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Acquire;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct ReadInput {
    sql: String,
}
#[derive(Debug, Serialize)]
pub struct ReadOutput {
    data: Option<Vec<Value>>,
}
pub async fn data_read_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
    axum::Json(item): axum::extract::Json<ReadInput>,
) -> AppResult<ReadOutput> {
    let organization_id = paper.organization.map(|o| o.id).unwrap();
    let rows = DataReader::read(ctx.storage.clone(), organization_id, item.sql).await;

    let output = ReadOutput { data: rows.ok() };
    AppResponse::ok(Some(output), None)
}
