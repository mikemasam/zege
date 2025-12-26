use anyhow::ensure;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::Extension;

use crate::{
    auth::{
        role::{NewRole, auth_create_role},
        team::NewTeam,
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
pub async fn auth_login(
    Extension(ctx): Extension<Arc<AppContext>>,
    axum::Json(item): axum::extract::Json<LoginCredentials>,
) -> AppResult<LoginResult> {
    let user = auth_login_verify_user_creds(ctx.storage.clone(), item).await?;
    let res = auth_login_make_paper(&user)?;
    let msg = format!("Welcome {}", user.name.unwrap_or_default());
    AppResponse::created(Some(res), Some(msg.as_str()))
}
