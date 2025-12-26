use anyhow::ensure;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::Extension;

use crate::{
    ctx::appcontext::AppContext,
    lib::auth::user::papers::{LoginCredentials, LoginResult, UserPaper},
    utils::http::{AppResponse, AppResult},
};
pub async fn auth_login(
    Extension(ctx): Extension<Arc<AppContext>>,
    axum::Json(item): axum::extract::Json<LoginCredentials>,
) -> AppResult<LoginResult> {
    let user = UserPaper::verify_creds(ctx.storage.clone(), item).await?;
    let res = UserPaper::login_paper(ctx.storage.clone(), &user).await?;
    let msg = format!("Welcome {}", user.name.unwrap_or_default());
    AppResponse::created(Some(res), Some(msg.as_str()))
}
