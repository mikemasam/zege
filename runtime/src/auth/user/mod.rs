mod resetpassword;
pub mod papers;
pub mod create;
use std::sync::Arc;

use anyhow::{Result, ensure};
use chrono::Duration;
use chrono::{DateTime, FixedOffset, Local};
use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Pool, Postgres, Sqlite, prelude::FromRow};
use std::{default, env};
use tokio::sync::Mutex;

use crate::auth::user::create::{auth_create_user, NewUser};
use crate::auth::user::papers::{auth_find_user_by_email, User};
use crate::auth::user::resetpassword::{ResetPasswordUserDto, auth_reset_user_password};
use crate::ctx::dbmanager::DbManager;
use crate::ctx::{appcontext::AppContext, dbmanager::DatabasePool};
use crate::utils::appenv::AppLogger;
use crate::utils::security::Security;


async fn auth_search_users(db: DbManager, pattern: Option<String>) -> Result<Vec<User>> {
    match db.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let pattern = format!("%{}%", pattern.unwrap_or_default());
            let users = sqlx::query_as::<_, User>(
                "select * from users where email ilike $1 or name ilike $2",
            )
            .bind(&pattern)
            .bind(&pattern)
            .fetch_all(pool)
            .await;
            Ok(users.unwrap())
        }
        DatabasePool::Sqlite(pool) => todo!("auth_search_users on sqlite"),
    }
}

#[derive(Debug, Subcommand, Clone, Serialize, Deserialize)]
pub enum UserCommands {
    Add {
        #[arg(long)]
        name: String,

        #[arg(long)]
        email: String,
    },
    Disable {
        #[arg(long)]
        email: String,
    },
    ResetPassword {
        #[arg(long)]
        email: String,
    },
    Search {
        #[arg(long)]
        pattern: Option<String>,
    },
}

async fn listUsers(app: AppContext, pattern: Option<String>) -> Result<()> {
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    let users = auth_search_users(db.clone(), pattern).await?;
    println!("{}", serde_json::to_string_pretty(&users)?);
    Ok(())
}

pub async fn auth_user_commands(ctx: AppContext, command: UserCommands) -> Result<()> {
    match command {
        UserCommands::Add { name, email } => {
            auth_create_user(
                ctx,
                NewUser {
                    name,
                    email,
                    password: "default".to_string(),
                },
            )
            .await?;
        }
        UserCommands::Search { pattern } => {
            listUsers(ctx, pattern).await?;
        }
        UserCommands::Disable { email } => todo!("user disable not implemented"),
        UserCommands::ResetPassword { email } => {
            auth_reset_user_password(
                ctx,
                ResetPasswordUserDto {
                    email,
                    current_password: None,
                    new_password: None,
                },
            )
            .await?;
        }
    };
    Ok(())
}
