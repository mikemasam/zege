#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::lib::auth::user::papers::UserPaper;
use crate::lib::services::{Service, auth_create_service};
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
    Extension(user): Extension<UserPaper>,
    axum::Json(item): axum::extract::Json<ServiceCreate>,
) -> AppResult<Service> {
    let service = auth_create_service(
        ctx.storage.clone(),
        crate::lib::services::NewService {
            name: item.name,
            label: item.label,
            description: item.description,
            user_id: user.id,
            organization_id: 1,
        },
    )
    .await?;
    AppResponse::created(Some(service), None)
}
