use anyhow::{Result, ensure};
use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    ctx::{appcontext::DbStorage, dbmanager::DatabasePool},
    utils::{appenv::AppLogger, security::Security},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserAccount {
    pub id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserPublicInfo {
    pub id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl UserAccount {
    pub async fn list(db: DbStorage, organization_id: i64) -> Result<Vec<UserPublicInfo>> {
        let pool = db.pool.clone();
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let exists_in = "SELECT 1 FROM organization_memberships AS e where e.organization_id = $1 AND e.user_id = u.id";
                let sql = format!(
                    "SELECT u.* FROM users AS u WHERE EXISTS ({exists_in}) ORDER BY name ASC"
                );
                let users = sqlx::query_as::<_, UserPublicInfo>(sql.as_str())
                    .bind(organization_id)
                    .fetch_all(pool)
                    .await?;
                Ok(users)
            }
            _ => todo!("auth_users_list on sqlite"),
        }
    }
    pub async fn find_by_email(storage: DbStorage, email: String) -> Result<UserAccount> {
        let pool = storage.pool.clone();
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let user = sqlx::query_as::<_, UserAccount>("select * from users where email = $1")
                    .bind(&email)
                    .fetch_one(pool)
                    .await;
                ensure!(user.is_ok(), "User with email {email} not found");
                Ok(user.unwrap())
            }
            _ => todo!("auth_find_user_by_email on db driver"),
        }
    }
    pub async fn find_by_id(db: DbStorage, id: i64) -> Result<UserAccount> {
        let pool = db.pool.clone();
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let user = sqlx::query_as::<_, UserAccount>("select * from users where id = $1")
                    .bind(id)
                    .fetch_one(pool)
                    .await;
                ensure!(user.is_ok(), "User with id {id} not found");
                Ok(user.unwrap())
            }
            DatabasePool::Sqlite(pool) => todo!("auth_find_user_by_id on sqlite"),
        }
    }

    pub async fn create(db: DbStorage, item: NewUser) -> Result<UserAccount> {
        let pool = db.pool.clone();
        ensure!(!item.name.is_empty(), "Name is required");
        ensure!(!item.email.is_empty(), "Email is required");
        ensure!(item.email.contains('@'), "invalid email");
        ensure!(!item.password.is_empty(), "Password is required");
        ensure!(
            item.password.len() >= 4,
            "Password length minimum is 4 characters"
        );
        let res = match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let dup = sqlx::query_as::<_, UserAccount>("select * from users where email = $1")
                    .bind(&item.email)
                    .fetch_optional(pool)
                    .await?;
                ensure!(
                    dup.is_none(),
                    "User with email already exists with password 6767"
                );
                let sql = "INSERT INTO users (email, name, password_hash, apikey_value, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *";
                let password = Security::hash_password(&item.password)?;
                let apikey_value = format!("zg{}", Uuid::now_v7().simple().to_string());
                let q = sqlx::query_as::<_, UserAccount>(sql)
                    .bind(&item.email)
                    .bind(&item.name)
                    .bind(password)
                    .bind(apikey_value)
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
}
