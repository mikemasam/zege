use crate::{
    ctx::appcontext::AppContext,
    lib::{auth::user::papers::UserPaper, organization::{Organization, OrganizationListItem}},
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn organizations_index_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
) -> AppResult<Vec<OrganizationListItem>> {
    let organizations = Organization::list(ctx.storage.clone(), Some(paper.id)).await?;
    AppResponse::ok(Some(organizations), None)
}
