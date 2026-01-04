use crate::{
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    lib::{auth::user::papers::UserPaper, buckets::Bucket},
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn buckets_index_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
) -> AppResult<Vec<Bucket>> {
    let buckets = Bucket::list(
        ctx.storage.clone(),
        paper.organization.map(|o| o.id).unwrap(),
    )
    .await?;
    AppResponse::ok(Some(buckets), None)
}
