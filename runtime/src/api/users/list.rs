use crate::{
    auth::impl::user::{auth_users_list, papers::User},
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn users_index_route(Extension(ctx): Extension<Arc<AppContext>>) -> AppResult<Vec<User>> {
    let users = auth_users_list(ctx.storage.clone()).await?;
    AppResponse::ok(Some(users), None)
}
