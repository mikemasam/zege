use crate::{
    ctx::appcontext::AppContext,
    lib::auth::user::{papers::UserPaper, user::{UserAccount, UserPublicInfo}},
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn users_index_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
) -> AppResult<Vec<UserPublicInfo>> {
    let users = UserAccount::list(
        ctx.storage.clone(),
        paper.organization.map(|o| o.id).unwrap(),
    )
    .await?;
    AppResponse::ok(Some(users), None)
}
