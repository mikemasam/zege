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
pub struct Service {
    pub id: i64,
    pub name: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub organization_id: i64,
    pub user_id: i64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewService {
    pub name: String,
    pub label: String,
    pub description: String,
    pub organization_id: i64,
    pub user_id: i64,
}
pub async fn auth_create_service(db: DbStorage, item: NewService) -> Result<Service> {
    let pool = db.pool.clone();
    ensure!(!item.name.is_empty(), "Name is required");
    ensure!(item.user_id > 0, "user id is required");
    ensure!(item.organization_id > 0, "organization id is required");
    match pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => todo!("auth_create_service on sqlite"),
        DatabasePool::Postgres(pool) => {
            let dup = sqlx::query_as::<_, Service>(
                "select * from services where name = $1 and organization_id = $2",
            )
            .bind(&item.name)
            .bind(item.organization_id)
            .fetch_optional(pool)
            .await?;
            println!("{:?}", dup);
            ensure!(dup.is_none(), "Service with the same key already exists");
            let sql = "INSERT INTO services (name, label, description, organization_id, user_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *";
            let q = sqlx::query_as::<_, Service>(sql)
                .bind(&item.name)
                .bind(&item.label)
                .bind(&item.description)
                .bind(item.organization_id)
                .bind(item.user_id)
                .bind(Local::now())
                .bind(Local::now());
            let service = q.fetch_one(pool).await?;
            println!("done creating service {:?}", service);
            Ok(service)
        }
    }
}

pub async fn auth_services_list(db: DbStorage) -> Result<Vec<Service>> {
    let pool = db.pool.clone();
    let sql = "SELECT * FROM services ORDER BY id DESC";
    let reports = match pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => sqlx::query_as::<_, Service>(sql).fetch_all(pool).await?,
        DatabasePool::Sqlite(pool) => todo!("auth_services_list"),
    };
    Ok(reports)
}

#[derive(Debug, Subcommand, Clone, Serialize, Deserialize)]
pub enum ServiceCommands {
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

async fn listServices(db: DbStorage, pattern: Option<String>) -> Result<()> {
    let pool = db.pool.clone();
    let services = match pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let pattern = format!("%{}%", pattern.unwrap_or_default());
            let services =
                sqlx::query_as::<_, Service>("select * from services where name ilike $1")
                    .bind(&pattern)
                    .fetch_all(pool)
                    .await;
            services.unwrap()
        }
        DatabasePool::Sqlite(pool) => todo!("listServices on sqlite"),
    };
    println!("{}", serde_json::to_string_pretty(&services)?);
    Ok(())
}
pub async fn auth_service_commands(ctx: Arc<AppContext>, command: ServiceCommands) -> Result<()> {
    match command {
        ServiceCommands::Add { name } => {
            //auth_create_service(ctx.storage.clone(), NewService { user_id: 0, name }).await?;
        }
        ServiceCommands::Search { pattern } => {
            listServices(ctx.storage.clone(), pattern).await;
        }
        ServiceCommands::Disable { id } => todo!("service disable not implemented"),
    };
    Ok(())
}
