use crate::{
    lib::services::{Service, auth_services_list},
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn services_index_route(
    Extension(ctx): Extension<Arc<AppContext>>,
) -> AppResult<Vec<Service>> {
    let services = auth_services_list(ctx.storage.clone()).await?;
    AppResponse::ok(Some(services), None)
}
