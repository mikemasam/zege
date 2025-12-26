use anyhow::ensure;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::Extension;

use crate::{
    auth::{
        role::{NewRole, auth_create_role},
        team::{NewTeam, auth_create_team},
        user::{
            create::{NewUser, auth_create_user},
            papers::{
                LoginCredentials, LoginResult, auth_login_make_paper, auth_login_verify_user_creds,
            },
        },
    },
    ctx::appcontext::AppContext,
    utils::http::{AppResponse, AppResult},
};

pub async fn auth_signup(
    Extension(ctx): Extension<Arc<AppContext>>,
    axum::Json(item): axum::extract::Json<NewUser>,
) -> AppResult<LoginResult> {
    let user = auth_create_user(ctx.storage.clone(), item).await?;
    let team = auth_create_team(
        ctx.storage.clone(),
        NewTeam {
            name: "Default Team".to_string(),
            user_id: user.id,
        },
    )
    .await?;
    let _ = auth_create_role(
        ctx.storage.clone(),
        NewRole {
            name: "Administrator".to_string(),
            description: "Administrator".to_string(),
            team_id: team.id,
        },
    )
    .await?;
    let res = auth_login_make_paper(&user)?;
    let msg = format!("Welcome {}", user.name.unwrap_or_default());
    AppResponse::created(Some(res), Some(msg.as_str()))
}
