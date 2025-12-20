use std::sync::Arc;

use anyhow::{Result, ensure};
use chrono::Duration;
use chrono::{DateTime, FixedOffset, Local};
use clap::{Args, Subcommand};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Pool, Postgres, Sqlite, prelude::FromRow};
use std::{default, env};
use tokio::sync::Mutex;

use crate::ctx::dbmanager::DbManager;
use crate::ctx::{appcontext::AppContext, dbmanager::DatabasePool};
use crate::utils::appenv::AppLogger;
use crate::utils::security::Security;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUser {
    name: String,
    email: String,
    password: String,
}
pub async fn auth_create_user(app: AppContext, item: NewUser) -> Result<User> {
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    ensure!(!item.name.is_empty(), "Name is required");
    ensure!(!item.email.is_empty(), "Email is required");
    ensure!(item.email.contains('@'), "invalid email");
    ensure!(!item.password.is_empty(), "Password is required");
    ensure!(
        item.password.len() >= 4,
        "Password length minimum is 4 characters"
    );
    let res = match db.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let dup = sqlx::query_as::<_, User>("select * from users where email = $1")
                .bind(&item.email)
                .fetch_optional(pool)
                .await?;
            //println!("{:?}", dup);
            ensure!(
                dup.is_none(),
                "User with email already exists with password 6767"
            );
            let sql = "INSERT INTO users (email, name, password_hash, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *";
            let password = Security::hash_password(&item.password)?;
            let q = sqlx::query_as::<_, User>(sql)
                .bind(&item.email)
                .bind(&item.name)
                .bind(password)
                .bind(Local::now())
                .bind(Local::now());
            let user = q.fetch_one(pool).await?;
            Ok(user)
        }
        DatabasePool::Sqlite(pool) => todo!("sqlite_auth_create_user on sqlite"),
    };
    if res.as_ref().is_ok() {
        AppLogger::log(format!(
            "new user added with id: {}",
            res.as_ref().unwrap().id
        ));
    }
    if res.as_ref().is_err() {
        println!("{:?}", res);
    }
    res
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginCredentials {
    email: String,
    password: String,
}

pub async fn auth_login_user(
    appcontext: Arc<Mutex<AppContext>>,
    item: LoginCredentials,
) -> Result<User> {
    let app = appcontext.lock().await;
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    let user = auth_find_user_by_email(db.clone(), item.email).await?;
    let valid = Security::verify_password(&item.password, &user.password_hash);
    ensure!(valid.is_ok(), "Invalid username or password");
    Ok(user)
}

async fn auth_find_user_by_email(db: DbManager, email: String) -> Result<User> {
    match db.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let user = sqlx::query_as::<_, User>("select * from users where email = $1")
                .bind(&email)
                .fetch_one(pool)
                .await;
            ensure!(user.is_ok(), "User with email {email} not found");
            Ok(user.unwrap())
        }
        DatabasePool::Sqlite(pool) => todo!("auth_find_user_by_email on sqlite"),
    }
}
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    token: String,
    user: LoginUserResult,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserResult {
    id: i64,
    name: Option<String>,
    email: Option<String>,
}
pub fn auth_login_session(user: &User) -> Result<LoginResult> {
    Ok(LoginResult {
        token: make_token(user)?,
        user: LoginUserResult {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
        },
    })
}
#[derive(Debug, Serialize, Deserialize)]
struct AuthClaims {
    pub sub: String,
    pub exp: usize,
}
fn make_token(user: &User) -> Result<String> {
    let secret = env::var("JWT_SECRET")?;
    let exp = Local::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = AuthClaims {
        sub: user.id.to_string(),
        exp,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
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
            .await;
        }
        UserCommands::Search { pattern } => {
            listUsers(ctx, pattern).await;
        }
        UserCommands::Disable { email } => todo!("user disable not implemented"),
    };
    Ok(())
}
