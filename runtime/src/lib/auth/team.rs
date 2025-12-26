use std::sync::Arc;

use anyhow::{Result, ensure};
use chrono::{DateTime, FixedOffset, Local};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Sqlite, prelude::FromRow};

use crate::ctx::{
    appcontext::{AppContext, DbStorage},
    dbmanager::{DatabasePool, DbPoolManager},
};

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
pub async fn auth_create_team(db: DbStorage, item: NewTeam) -> Result<Team> {
    let pool = db.pool.clone();
    ensure!(!item.name.is_empty(), "Name is required");
    ensure!(item.user_id > 0, "User id is required");
    match pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => todo!("auth_create_team on sqlite"),
        DatabasePool::Postgres(pool) => {
            let dup =
                sqlx::query_as::<_, Team>("select * from teams where name = $1 and user_id = $2")
                    .bind(&item.name)
                    .bind(item.user_id)
                    .fetch_optional(pool)
                    .await?;
            println!("{:?}", dup);
            ensure!(dup.is_none(), "Team with name already exists");
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
    }
}

pub async fn auth_teams_list(db: DbStorage) -> Result<Vec<Team>> {
    let pool = db.pool.clone();
    let sql = "SELECT * FROM teams ORDER BY id DESC";
    let reports = match pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => sqlx::query_as::<_, Team>(sql).fetch_all(pool).await?,
        DatabasePool::Sqlite(pool) => todo!("auth_teams_list"),
    };
    Ok(reports)
}

#[derive(Debug, Subcommand, Clone, Serialize, Deserialize)]
pub enum TeamCommands {
    Add {
        #[arg(long)]
        name: String,
    },
    Disable {
        #[arg(long)]
        id: i32,
    },
    Search {
        #[arg(long)]
        pattern: Option<String>,
    },
}

async fn listTeams(db: DbStorage, pattern: Option<String>) -> Result<()> {
    let pool = db.pool.clone();
    let teams = match pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let pattern = format!("%{}%", pattern.unwrap_or_default());
            let teams = sqlx::query_as::<_, Team>("select * from teams where name ilike $1")
                .bind(&pattern)
                .fetch_all(pool)
                .await;
            teams.unwrap()
        }
        DatabasePool::Sqlite(pool) => todo!("listTeams on sqlite"),
    };
    println!("{}", serde_json::to_string_pretty(&teams)?);
    Ok(())
}
pub async fn auth_team_commands(ctx: Arc<AppContext>, command: TeamCommands) -> Result<()> {
    match command {
        TeamCommands::Add { name } => {
            auth_create_team(ctx.storage.clone(), NewTeam { user_id: 0, name }).await?;
        }
        TeamCommands::Search { pattern } => {
            listTeams(ctx.storage.clone(), pattern).await;
        }
        TeamCommands::Disable { id } => todo!("team disable not implemented"),
    };
    Ok(())
}
