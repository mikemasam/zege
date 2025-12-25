use anyhow::ensure;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{Extension, http::HeaderMap};

use crate::{
    api_ensure,
    auth::{
        role::{NewRole, auth_create_role},
        team::{NewTeam, auth_create_team},
        user::{
            create::{NewUser, auth_create_user},
            papers::{
                LoginCredentials, LoginResult, UserPaper, auth_find_user_by_id,
                auth_login_verify_user_creds, auth_login_verify_user_token, verify_jwt,
            },
        },
    },
    ctx::appcontext::AppContext,
    utils::http::{AppResponse, AppResult},
};

pub async fn papers_please(Extension(paper): Extension<UserPaper>) -> AppResult<UserPaper> {
    AppResponse::ok(Some(paper), None)
}
