#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::dto::logevent::{LogEvent, ZegeEventRow};
use crate::lib::auth::user::papers::UserPaper;
use crate::utils::http::{AppResponse, AppResult};
use axum::{Extension, extract::Query};
use futures::StreamExt;
use serde::Deserialize;
use sqlx::{Pool, Postgres, QueryBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    search: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
    event_name: Option<String>,
    host: Option<String>,
}

pub async fn list_events_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
    Query(query_params): Query<QueryParams>,
) -> AppResult<Vec<LogEvent>> {
    let rows = match ctx.storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            fetch_postgres(
                pool,
                paper.organization.map(|o| o.id).unwrap(),
                query_params,
            )
            .await
        }
        _ => todo!("list_events_route"),
    };
    AppResponse::ok(Some(rows), None)
}

async fn fetch_postgres(
    pool: &Pool<Postgres>,
    organization_id: i64,
    query_params: QueryParams,
) -> Vec<LogEvent> {
    let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM zege_events WHERE ");
    qb.push("event_organization_id = ")
        .push_bind(organization_id);
    if let Some(search) = query_params.search.filter(|s| !s.trim().is_empty()) {
        let fields = ["event_name", "service", "host"];
        let mut is_empty = true;
        for (i, (field, value)) in fields.iter().zip(search.split(':')).enumerate() {
            if (value == "*") {
                continue;
            }
            if !is_empty {
                qb.push(" AND ");
            } else {
                qb.push(" AND ( ");
            }
            qb.push(format!("{} ILIKE ", field))
                .push_bind(format!("%{}%", value));
            is_empty = false;
        }
        if !is_empty {
            qb.push(")");
        }
    }
    qb.push(" ORDER BY id DESC");
    qb.push(" LIMIT ")
        .push_bind(Into::<i64>::into(query_params.per_page.unwrap_or(50)));
    qb.push(" OFFSET ").push_bind(Into::<i64>::into(
        query_params.page.unwrap_or(0) * query_params.per_page.unwrap_or(15),
    ));

    // Build a typed query
    let query = qb.build_query_as::<ZegeEventRow>();

    // Execute
    let mut rows = query.fetch(pool);
    let mut results = Vec::new();

    while let Some(row) = rows.next().await {
        match row {
            Ok(r) => {
                results.push(r.to_event());
            }
            Err(e) => {
                eprintln!("error fetching row: {:?}", e);
                break;
            }
        }
    }

    results
}

