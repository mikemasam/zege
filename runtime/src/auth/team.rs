use std::sync::Arc;

use anyhow::{Result, ensure};
use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Sqlite, prelude::FromRow};
use tokio::sync::Mutex;

use crate::ctx::{appcontext::AppContext, dbmanager::DatabasePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub name: String,
    pub user_id: i64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewTeam {
    pub name: String,
    pub user_id: i64,
}
pub async fn auth_create_team(appcontext: Arc<Mutex<AppContext>>, item: NewTeam) -> Result<Team> {
    let app = appcontext.lock().await;
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    let item = validate(item)?;
    match db.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => sqlite_auth_create_team(pool, item).await,
        DatabasePool::Postgres(pool) => pgsql_auth_create_team(pool, item).await,
    }
}
fn validate(item: NewTeam) -> Result<NewTeam> {
    ensure!(!item.name.is_empty(), "Email is required");
    ensure!(item.user_id > 0, "User id is required");
    Ok(item)
}
async fn pgsql_auth_create_team(pool: &Pool<Postgres>, item: NewTeam) -> Result<Team> {
    let dup = sqlx::query_as::<_, Team>("select * from teams where name = $1 and user_id = $2")
        .bind(&item.name)
        .bind(item.user_id)
        .fetch_optional(pool)
        .await?;
    println!("{:?}", dup);
    ensure!(
        dup.is_none(),
        "Team with name already exists"
    );
    let sql = "INSERT INTO teams (name, user_id, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *";
    let q = sqlx::query_as::<_, Team>(sql)
        .bind(&item.name)
        .bind(item.user_id)
        .bind(Local::now())
        .bind(Local::now());
    let team = q.fetch_one(pool).await?;
    println!("done creating team {:?}", team);
    Ok(team)
}

async fn sqlite_auth_create_team(pool: &Pool<Sqlite>, item: NewTeam) -> Result<Team> {
    todo!("signup on sqlite");
}
