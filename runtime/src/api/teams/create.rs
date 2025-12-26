#![allow(dead_code)]
use crate::lib::auth::team::{Team, auth_create_team};
use crate::lib::auth::user::papers::UserPaper;
use crate::ctx::appcontext::AppContext;
use crate::ctx::dbmanager::DatabasePool;
use crate::utils::http::{AppResponse, AppResult};
use axum::Extension;
use chrono::Local;
use serde::Deserialize;
use sqlx::{PgPool, SqlitePool};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct TeamCreate {
    name: String,
}

pub async fn teams_create_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Extension(user): Extension<UserPaper>,
    axum::Json(item): axum::extract::Json<TeamCreate>,
) -> AppResult<Team> {
    let team = auth_create_team(
        ctx.storage.clone(),
        crate::auth::team::NewTeam {
            name: item.name,
            user_id: user.id,
        },
    )
    .await?;
    AppResponse::created(Some(team), None)
}
