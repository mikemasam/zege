use anyhow::ensure;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::Extension;

use crate::{
    api_ensure, appconfig,
    ctx::appcontext::AppContext,
    lib::{
        auth::{
            role::{NewRole, Role},
            user::{
                papers::{LoginCredentials, LoginResult, UserPaper},
                user::{NewUser, UserAccount},
            },
        },
        organization::{
            NewOrganization, NewOrganizationMembership, Organization, OrganizationMembership,
            SwitchOrganizationMembership,
        },
    },
    utils::http::{AppResponse, AppResult},
};

pub async fn auth_signup(
    Extension(ctx): Extension<Arc<AppContext>>,
    axum::Json(item): axum::extract::Json<NewUser>,
) -> AppResult<LoginResult> {
    let enabled = appconfig!()
        .feature
        .as_ref()
        .and_then(|f| f.create_organization.as_deref())
        .map(|v| v.to_lowercase())
        .map(|v| v != "no" && v != "false")
        .unwrap_or(false);
    api_ensure!(
        enabled,
        "Signup not available at the moment, this can be enable on config.{yaml, json, ...}"
    );

    //TODO: handle db rollback
    let user = UserAccount::create(ctx.storage.clone(), item).await?;
    let organization = Organization::create(
        ctx.storage.clone(),
        NewOrganization {
            name: "Default Organization".to_string(),
            user_id: user.id,
        },
    )
    .await?;
    let res = UserPaper::login_paper(ctx.storage.clone(), &user).await?;
    let msg = format!("Welcome {}", user.name.unwrap_or_default());
    AppResponse::created(Some(res), Some(msg.as_str()))
}
