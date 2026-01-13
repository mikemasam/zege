#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::lib::auth::user::papers::UserPaper;
use crate::lib::buckets::Bucket;
use crate::utils::appconfig::AppFeature;
use crate::utils::http::{AppResponse, AppResult};
use crate::{api_ensure, appconfig};
use axum::Extension;
use chrono::Local;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct BucketCreate {
    name: String,
    description: String,
}

pub async fn buckets_create_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
    axum::Json(item): axum::extract::Json<BucketCreate>,
) -> AppResult<Bucket> {
    let enabled = appconfig!().feature_enabled(AppFeature::CreateBucket);
    api_ensure!(enabled, "create_bucket not available at the moment");

    let bucket = Bucket::create(
        ctx.storage.clone(),
        crate::lib::buckets::NewBucket {
            name: item.name,
            description: item.description,
            user_id: paper.id,
            organization_id: paper.organization.map(|o| o.id).unwrap(),
        },
    )
    .await?;
    AppResponse::created(Some(bucket), None)
}
