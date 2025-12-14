use anyhow::{Result, ensure};
use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Sqlite, prelude::FromRow};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::ctx::{appcontext::AppContext, dbmanager::DatabasePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub name: String,
    pub description: String,
    pub team_id: i64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewRole {
    pub name: String,
    pub description: String,
    pub team_id: i64,
}
pub async fn auth_create_role(appcontext: Arc<Mutex<AppContext>>, item: NewRole) -> Result<Role> {
    let app = appcontext.lock().await;
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    let item = validate(item)?;
    match db.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => sqlite_auth_create_role(pool, item).await,
        DatabasePool::Postgres(pool) => pgsql_auth_create_role(pool, item).await,
    }
}
fn validate(item: NewRole) -> Result<NewRole> {
    ensure!(!item.name.is_empty(), "Name is required");
    ensure!(item.team_id > 0, "team_id is required");
    Ok(item)
}
async fn pgsql_auth_create_role(pool: &Pool<Postgres>, item: NewRole) -> Result<Role> {
    let dup = sqlx::query_as::<_, Role>("select * from roles where name = $1 and team_id = $2")
        .bind(&item.name)
        .bind(item.team_id)
        .fetch_optional(pool)
        .await?;
    println!("{:?}", dup);
    ensure!(dup.is_none(), "Role with name already exists");
    let sql = "INSERT INTO roles (name, description, team_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *";
    let q = sqlx::query_as::<_, Role>(sql)
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.team_id)
        .bind(Local::now())
        .bind(Local::now());
    let role = q.fetch_one(pool).await?;
    println!("done creating role {:?}", role);
    Ok(role)
}

async fn sqlite_auth_create_role(pool: &Pool<Sqlite>, item: NewRole) -> Result<Role> {
    todo!("signup on sqlite");
}
