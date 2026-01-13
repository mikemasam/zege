use anyhow::ensure;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{Extension, http::HeaderMap};

use crate::{
    ctx::appcontext::AppContext,
    lib::{
        auth::user::papers::UserPaper,
        organization::{Organization, OrganizationMembership, SwitchOrganizationMembership},
    },
    utils::http::{AppResponse, AppResult},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct SwitchForm {
    org_id: i64,
}
pub async fn switch_organization(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
    axum::Json(item): axum::extract::Json<SwitchForm>,
) -> AppResult<()> {
    OrganizationMembership::switch(
        ctx.storage.clone(),
        SwitchOrganizationMembership {
            organization_id: item.org_id,
            user_id: paper.id,
        },
    )
    .await?;
    AppResponse::ok(None, Some("Switched"))
}
