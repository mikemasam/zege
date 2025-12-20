use std::sync::Arc;
use tokio::sync::Mutex;

use axum::Extension;

use crate::{
    auth::{
        role::{NewRole, auth_create_role},
        team::{NewTeam, auth_create_team},
        user::{
            LoginCredentials, LoginResult, NewUser, auth_create_user, auth_login_session,
            auth_login_user,
        },
    },
    ctx::appcontext::AppContext,
    utils::http::{AppResponse, AppResult},
};

pub async fn auth_signup(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    axum::Json(item): axum::extract::Json<NewUser>,
) -> AppResult<LoginResult> {
    let app = appcontext.lock().await;
    let user = auth_create_user(app.clone(), item).await?;
    let team = auth_create_team(
        app.clone(),
        NewTeam {
            name: "Default Team".to_string(),
            user_id: user.id,
        },
    )
    .await?;
    let _ = auth_create_role(
        appcontext.clone(),
        NewRole {
            name: "Administrator".to_string(),
            description: "Administrator".to_string(),
            team_id: team.id,
        },
    )
    .await?;
    let res = auth_login_session(&user)?;
    let msg = format!("Welcome {}", user.name.unwrap_or_default());
    AppResponse::created(Some(res), Some(msg.as_str()))
}

pub async fn auth_login(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    axum::Json(item): axum::extract::Json<LoginCredentials>,
) -> AppResult<LoginResult> {
    let user = auth_login_user(appcontext, item).await?;
    let res = auth_login_session(&user)?;
    let msg = format!("Welcome {}", user.name.unwrap_or_default());
    AppResponse::created(Some(res), Some(msg.as_str()))
}
