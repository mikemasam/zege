pub mod papers;
pub mod user;
mod resetpassword;
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

use crate::lib::auth::user::resetpassword::{ResetPasswordUserDto};
use crate::ctx::appcontext::DbStorage;
use crate::ctx::{appcontext::AppContext, dbmanager::DatabasePool};
use crate::lib::auth::user::user::{NewUser, User};

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

async fn listUsers(storage: DbStorage, pattern: Option<String>) -> Result<()> {
    let users = match storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let pattern = format!("%{}%", pattern.unwrap_or_default());
            let users = sqlx::query_as::<_, User>(
                "select * from users where email ilike $1 or name ilike $2",
            )
            .bind(&pattern)
            .bind(&pattern)
            .fetch_all(pool)
            .await;
            users.unwrap()
        }
        DatabasePool::Sqlite(pool) => todo!("auth_search_users on sqlite"),
    };
    println!("{}", serde_json::to_string_pretty(&users)?);
    Ok(())
}

pub async fn auth_user_commands(ctx: Arc<AppContext>, command: UserCommands) -> Result<()> {
    match command {
        UserCommands::Add { name, email } => {
            User::create(
                ctx.storage.clone(),
                NewUser {
                    name,
                    email,
                    password: "default".to_string(),
                },
            )
            .await?;
        }
        UserCommands::Search { pattern } => {
            listUsers(ctx.storage.clone(), pattern).await?;
        }
        UserCommands::Disable { email } => todo!("user disable not implemented"),
        UserCommands::ResetPassword { email } => {
            User::reset_password(
                ctx.storage.clone(),
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
