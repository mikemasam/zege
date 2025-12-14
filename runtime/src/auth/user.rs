use std::sync::Arc;

use anyhow::{Result, ensure};
use chrono::Duration;
use chrono::{DateTime, FixedOffset, Local};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Sqlite, prelude::FromRow};
use std::{default, env};
use tokio::sync::Mutex;

use crate::utils::security::Security;
use crate::{
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
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
pub async fn auth_create_user(appcontext: Arc<Mutex<AppContext>>, item: NewUser) -> Result<User> {
    let app = appcontext.lock().await;
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    let item = validate(item)?;
    match db.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => sqlite_auth_create_user(pool, item).await,
        DatabasePool::Postgres(pool) => pgsql_auth_create_user(pool, item).await,
    }
}
fn validate(item: NewUser) -> Result<NewUser> {
    ensure!(!item.name.is_empty(), "Name is required");
    ensure!(!item.email.is_empty(), "Email is required");
    ensure!(item.email.contains('@'), "invalid email");
    ensure!(!item.password.is_empty(), "Password is required");
    ensure!(
        item.password.len() >= 4,
        "Password length minimum is 4 characters"
    );
    Ok(item)
}
async fn pgsql_auth_create_user(pool: &Pool<Postgres>, item: NewUser) -> Result<User> {
    let dup = sqlx::query_as::<_, User>("select * from users where email = $1")
        .bind(&item.email)
        .fetch_optional(pool)
        .await?;
    println!("{:?}", dup);
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
    println!("done creating user {:?}", user);
    Ok(user)
}

async fn sqlite_auth_create_user(pool: &Pool<Sqlite>, item: NewUser) -> Result<User> {
    todo!("signup on sqlite");
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
    let user = match db.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => sqlite_auth_find_user_by_email(pool, item.email).await?,
        DatabasePool::Postgres(pool) => pgsql_auth_find_user_by_email(pool, item.email).await?,
    };
    let valid = Security::verify_password(&item.password, &user.password_hash);
    ensure!(valid.is_ok(), "Invalid username or password");
    Ok(user)
}

async fn pgsql_auth_find_user_by_email(pool: &Pool<Postgres>, email: String) -> Result<User> {
    let user = sqlx::query_as::<_, User>("select * from users where email = $1")
        .bind(&email)
        .fetch_one(pool)
        .await;
    ensure!(user.is_ok(), "User with email {email} not found");
    Ok(user.unwrap())
}

async fn sqlite_auth_find_user_by_email(pool: &Pool<Sqlite>, email: String) -> Result<User> {
    todo!("signup on sqlite");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    token: String,
    user: LoginUserResult,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserResult {
    id: i64,
    name: String,
    email: String,
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
