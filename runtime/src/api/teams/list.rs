use crate::{
    lib::auth::team::{Team, auth_teams_list},
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn teams_index_route(
    Extension(ctx): Extension<Arc<AppContext>>,
) -> AppResult<Vec<Team>> {
    let teams = auth_teams_list(ctx.storage.clone()).await?;
    AppResponse::ok(Some(teams), None)
}
