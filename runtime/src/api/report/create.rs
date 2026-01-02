use crate::api::report::list::ZegeReport;
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::lib::auth::user::papers::UserPaper;
use crate::utils::http::{AppResponse, AppResult};
use axum::Extension;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct ReportCreate {
    id: Option<i64>,
    report_name: String,
    report_sql: String,
    report_type: String,
}

pub async fn report_create_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
    axum::Json(item): axum::extract::Json<ReportCreate>,
) -> AppResult<ZegeReport> {
    let report = match ctx.storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            if (item.id.is_some()) {
                pgsql_update(pool, paper.organization.map(|o| o.id).unwrap(), item).await?
            } else {
                pgsql_write(
                    pool,
                    paper.organization.map(|o| o.id).unwrap(),
                    paper.id,
                    item,
                )
                .await?
            }
        }
        _ => todo!("report_sqlite_write"),
    };
    AppResponse::created(Some(report), None)
}

async fn pgsql_write(
    pool: &PgPool,
    organization_id: i64,
    user_id: i64,
    item: ReportCreate,
) -> Result<ZegeReport, sqlx::Error> {
    let sql = "INSERT INTO reports (report_name, report_type, report_sql, organization_id, user_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) RETURNING *";
    let q = sqlx::query_as::<_, ZegeReport>(sql)
        .bind(&item.report_name)
        .bind(item.report_type)
        .bind(item.report_sql)
        .bind(organization_id)
        .bind(user_id);
    q.fetch_one(pool).await
}

async fn pgsql_update(
    pool: &PgPool,
    organization_id: i64,
    item: ReportCreate,
) -> Result<ZegeReport, sqlx::Error> {
    let id = item.id.as_ref();
    let sql = "UPDATE reports SET report_name = $1, report_type = $2, report_sql = $3, updated_at = NOW() WHERE id = $4 and organization_id = $5  RETURNING *";
    let q = sqlx::query_as::<_, ZegeReport>(sql)
        .bind(&item.report_name)
        .bind(item.report_type)
        .bind(item.report_sql)
        .bind(item.id)
        .bind(organization_id);
    q.fetch_one(pool).await
}
