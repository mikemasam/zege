use std::sync::Arc;

use axum::Extension;
use serde::{Deserialize, Serialize};

use crate::{
    ctx::appcontext::AppContext,
    lib::{
        auth::user::{
            papers::UserPaper,
            user::{NewUser, UserAccount, UserPublicInfo},
        },
        organization::{NewOrganizationMembership, Organization, OrganizationMembership},
    },
    utils::http::{AppResponse, AppResult},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewOrganizationUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn users_create_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
    axum::Json(item): axum::extract::Json<NewOrganizationUser>,
) -> AppResult<UserPaper> {
    //TODO: handle db rollback
    let user = UserAccount::create(
        ctx.storage.clone(),
        NewUser {
            name: item.name,
            email: item.email,
            password: item.password,
        },
    )
    .await?;
    let db = ctx.storage.clone();
    OrganizationMembership::create(
        db,
        NewOrganizationMembership {
            organization_id: paper.organization.map(|o| o.id).unwrap(),
            user_id: user.id,
            role_id: paper.role.map(|r| r.id).unwrap(),
        },
    )
    .await?;
    AppResponse::created(
        Some(UserPaper {
            id: user.id,
            name: user.name,
            email: user.email,
            organization: None,
            role: None,
        }),
        Some("User added"),
    )
}
