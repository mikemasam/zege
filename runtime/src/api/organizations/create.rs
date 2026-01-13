#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::lib::auth::user::papers::UserPaper;
use crate::lib::organization::{NewOrganization, Organization};
use crate::utils::appconfig::AppFeature;
use crate::utils::http::{AppResponse, AppResult};
use crate::{api_ensure, appconfig};
use axum::Extension;
use chrono::Local;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct OrganizationCreate {
    name: String,
}

pub async fn organizations_create_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(user): Extension<UserPaper>,
    axum::Json(item): axum::extract::Json<OrganizationCreate>,
) -> AppResult<Organization> {
    let enabled = appconfig!().feature_enabled(AppFeature::CreateOrganization);
    api_ensure!(enabled, "create_organization not available at the moment");

    let organization = Organization::create(
        ctx.storage.clone(),
        NewOrganization {
            name: item.name,
            user_id: user.id,
        },
    )
    .await?;
    AppResponse::created(Some(organization), None)
}
