use anyhow::ensure;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::Extension;

use crate::{
    ctx::appcontext::AppContext,
    lib::{
        auth::{
            role::{NewRole, Role},
            user::{
                papers::{LoginCredentials, LoginResult, UserPaper},
                user::{NewUser, User},
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
    //TODO: handle db rollback
    let user = User::create(ctx.storage.clone(), item).await?;
    let organization = Organization::create(
        ctx.storage.clone(),
        NewOrganization {
            name: "Default Organization".to_string(),
            user_id: user.id,
        },
    )
    .await?;
    let role = Role::create(
        ctx.storage.clone(),
        NewRole {
            name: "Administrator".to_string(),
            description: "Administrator".to_string(),
            organization_id: organization.id,
        },
    )
    .await?;
    OrganizationMembership::create(
        ctx.storage.clone(),
        NewOrganizationMembership {
            organization_id: organization.id,
            user_id: user.id,
            role_id: role.id,
        },
    )
    .await?;
    Organization::switch(
        ctx.storage.clone(),
        SwitchOrganizationMembership {
            organization_id: organization.id,
            user_id: user.id,
        },
    )
    .await?;
    let res = UserPaper::login_paper(ctx.storage.clone(), &user).await?;
    let msg = format!("Welcome {}", user.name.unwrap_or_default());
    AppResponse::created(Some(res), Some(msg.as_str()))
}
