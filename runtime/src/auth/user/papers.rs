use std::env;
use std::sync::Arc;

use anyhow::{Result, ensure};
use chrono::{DateTime, Duration, FixedOffset, Local};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tokio::sync::Mutex;

use crate::{
    ctx::{
        appcontext::AppContext,
        dbmanager::{DatabasePool, DbManager},
    },
    utils::security::Security,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPaper {
    id: i64,
    name: Option<String>,
    email: Option<String>,
}

impl UserPaper {
    pub fn fromUser(user: &User) -> Self {
        return UserPaper {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    token: String,
    user: UserPaper,
}

pub async fn auth_find_user_by_email(db: DbManager, email: String) -> Result<User> {
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

pub async fn auth_find_user_by_id(db: DbManager, id: i64) -> Result<User> {
    match db.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let user = sqlx::query_as::<_, User>("select * from users where id = $1")
                .bind(id)
                .fetch_one(pool)
                .await;
            ensure!(user.is_ok(), "User with id {id} not found");
            Ok(user.unwrap())
        }
        DatabasePool::Sqlite(pool) => todo!("auth_find_user_by_id on sqlite"),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: i64,
    pub exp: usize,
}
pub fn generate_jwt(user: &User) -> Result<String> {
    let secret = env::var("JWT_SECRET")?;
    let exp = Local::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = AuthClaims { sub: user.id, exp };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}
pub fn verify_jwt(token: &str) -> Result<AuthClaims> {
    let secret = std::env::var("JWT_SECRET")?;
    let mut validation = Validation::default();
    validation.validate_exp = true;
    let token_data = decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims)
}
pub fn auth_login_make_paper(user: &User) -> Result<LoginResult> {
    Ok(LoginResult {
        token: generate_jwt(user)?,
        user: UserPaper::fromUser(user),
    })
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginCredentials {
    email: String,
    password: String,
}

pub async fn auth_login_verify_user_creds(
    appcontext: Arc<Mutex<AppContext>>,
    creds: LoginCredentials,
) -> Result<User> {
    let app = appcontext.lock().await;
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    let rs_user = auth_find_user_by_email(db.clone(), creds.email).await;
    ensure!(rs_user.is_ok(), "Invalid username or password");
    let user = rs_user.unwrap();
    let valid = Security::verify_password(&creds.password, &user.password_hash);
    ensure!(valid.is_ok(), "Invalid username or password");
    Ok(user)
}


pub async fn auth_login_verify_user_token(
    app: Arc<AppContext>,
    token: &str,
) -> Result<UserPaper> {
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    let claims = verify_jwt(token.replace("Bearer ", "").as_str())?;
    let rs_user = auth_find_user_by_id(db.clone(), claims.sub).await;
    ensure!(rs_user.is_ok(), "unauthorized");
    let user = rs_user.unwrap();
    Ok(UserPaper::fromUser(&user))
}
