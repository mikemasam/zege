use crate::{
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    lib::{auth::user::papers::UserPaper, services::Service},
    utils::http::{AppResponse, AppResult},
};
use axum::extract::Extension;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn services_index_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
) -> AppResult<Vec<Service>> {
    let services = Service::list(
        ctx.storage.clone(),
        paper.organization.map(|o| o.id).unwrap(),
    )
    .await?;
    AppResponse::ok(Some(services), None)
}
