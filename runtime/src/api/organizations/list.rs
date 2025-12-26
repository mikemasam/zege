use crate::{
    ctx::appcontext::AppContext,
    lib::organization::Organization,
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn organizations_index_route(
    Extension(ctx): Extension<Arc<AppContext>>,
) -> AppResult<Vec<Organization>> {
    let organizations = Organization::list(ctx.storage.clone()).await?;
    AppResponse::ok(Some(organizations), None)
}
