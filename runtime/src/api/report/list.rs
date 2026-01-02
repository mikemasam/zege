use crate::{
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    lib::auth::user::papers::UserPaper,
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ZegeReport {
    pub id: i64,
    pub report_name: String,
    pub report_type: String,
    pub report_sql: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub organization_id: i64,
    pub user_id: i64,
}

pub async fn report_index_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
) -> AppResult<Vec<ZegeReport>> {
    let reports = match ctx.storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let sql = "SELECT * FROM reports where organization_id = $1 ORDER BY id DESC";
            sqlx::query_as::<_, ZegeReport>(sql)
                .bind(paper.organization.map(|o| o.id).unwrap())
                .fetch_all(pool)
                .await
        }
        _ => todo!("sqlite report_index_route"),
    };
    AppResponse::ok(reports.ok(), None)
}
