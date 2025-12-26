use crate::{
    ctx::appcontext::AppContext,
    lib::auth::user::user::User,
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn users_index_route(Extension(ctx): Extension<Arc<AppContext>>) -> AppResult<Vec<User>> {
    let users = User::list(ctx.storage.clone()).await?;
    AppResponse::ok(Some(users), None)
}
