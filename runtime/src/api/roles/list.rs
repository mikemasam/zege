use crate::{
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    lib::auth::{
        role::{Role, auth_roles_list},
        user::papers::UserPaper,
    },
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn roles_index_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
) -> AppResult<Vec<Role>> {
    let roles = auth_roles_list(
        ctx.storage.clone(),
        paper.organization.map(|o| o.id).unwrap(),
    )
    .await?;
    AppResponse::ok(Some(roles), None)
}
