use crate::{
    auth::role::{auth_roles_list, Role},
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn roles_index_route(Extension(ctx): Extension<Arc<AppContext>>) -> AppResult<Vec<Role>> {
    let roles = auth_roles_list(ctx.storage.clone()).await?;
    AppResponse::ok(Some(roles), None)
}
