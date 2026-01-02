#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::lib::auth::user::papers::UserPaper;
use crate::lib::services::Service;
use crate::utils::http::{AppResponse, AppResult};
use axum::Extension;
use chrono::Local;
use serde::Deserialize;
use sqlx::{PgPool, SqlitePool};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct ServiceCreate {
    name: String,
    label: String,
    description: String,
}

pub async fn services_create_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
    axum::Json(item): axum::extract::Json<ServiceCreate>,
) -> AppResult<Service> {
    let service = Service::create(
        ctx.storage.clone(),
        crate::lib::services::NewService {
            name: item.name,
            label: item.label,
            description: item.description,
            user_id: paper.id,
            organization_id: paper.organization.map(|o| o.id).unwrap(),
        },
    )
    .await?;
    AppResponse::created(Some(service), None)
}
