#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::dto::logevent::{LogEvent, ZegeEventRow};
use crate::lib::auth::user::papers::UserPaper;
use crate::utils::http::{AppResponse, AppResult, DataCursor};
use anyhow::Result;
use axum::{Extension, extract::Query};
use chrono::{DateTime, Duration, FixedOffset, Local};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{Pool, Postgres, QueryBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    from: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SummaryData {
    label: String,
    items: Vec<SummaryItem>,
}
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct SummaryItem {
    label: String,
    count: i64,
}
pub async fn summary_events_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
    Query(query_params): Query<QueryParams>,
) -> AppResult<Vec<SummaryData>> {
    let data = match ctx.storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            fetch_postgres(
                pool,
                paper.organization.map(|o| o.id).unwrap(),
                query_params,
            )
            .await?
        }
        _ => todo!("summary_events_route"),
    };
    AppResponse::ok(Some(data), None)
}

async fn fetch_postgres(
    pool: &Pool<Postgres>,
    organization_id: i64,
    query_params: QueryParams,
) -> Result<Vec<SummaryData>> {
    let mut data: Vec<SummaryData> = vec![];
    let from = query_params
        .from
        .unwrap_or((Local::now() - Duration::minutes(30)).into());
    let cond = "WHERE event_created_at > $1 AND event_organization_id = $2";
    {
        let total_services = sqlx::query_as::<_, SummaryItem>(
            format!(
                "SELECT service as label, count(zege_events.id) as count FROM zege_events {} group by service",
                cond
            )
            .as_str(),
        )
        .bind(from)
        .bind(organization_id)
        .fetch_all(pool)
        .await?;
        data.push(SummaryData {
            label: "Services".to_string(),
            items: total_services,
        });
    }

    {
        let total_events = sqlx::query_as::<_, SummaryItem>(
            format!("SELECT event_name as label, count(zege_events.id) as count FROM zege_events {} group by event_name", cond
        ).as_str()).bind(from)
        .bind(organization_id)
        .fetch_all(pool)
        .await?;
        data.push(SummaryData {
            label: "Events".to_string(),
            items: total_events,
        });
    }

    {
        let sql = format!(
            "SELECT b.name as label, count(zege_events.id) as count FROM zege_events join buckets as b on b.id = event_bucket_id {} group by b.id",
            cond
        );
        let total_buckets = sqlx::query_as::<_, SummaryItem>(sql.as_str())
            .bind(from)
            .bind(organization_id)
            .fetch_all(pool)
            .await?;
        data.push(SummaryData {
            label: "Buckets".to_string(),
            items: total_buckets,
        });
    }

    Ok(data)
}
