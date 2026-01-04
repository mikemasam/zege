use std::sync::Arc;

use anyhow::{Result, ensure};
use chrono::{DateTime, FixedOffset, Local};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Sqlite, prelude::FromRow};
use uuid::Uuid;

use crate::ctx::{
    appcontext::{AppContext, DbStorage},
    dbmanager::{DatabasePool, DbPoolManager},
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Bucket {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub organization_id: i64,
    pub user_id: i64,
    pub bucket_key: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewBucket {
    pub name: String,
    pub description: String,
    pub organization_id: i64,
    pub user_id: i64,
}
impl Bucket {
    pub async fn create(db: DbStorage, item: NewBucket) -> Result<Bucket> {
        let pool = db.pool.clone();
        ensure!(!item.name.is_empty(), "Name is required");
        ensure!(item.user_id > 0, "user id is required");
        ensure!(item.organization_id > 0, "organization id is required");
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let dup = sqlx::query_as::<_, Bucket>(
                    "select * from buckets where name = $1 and organization_id = $2",
                )
                .bind(&item.name)
                .bind(item.organization_id)
                .fetch_optional(pool)
                .await?;
                ensure!(dup.is_none(), "Bucket with the same key already exists");
                let bucket_key = format!("zgb{}", Uuid::now_v7().simple().to_string());
                let sql = "INSERT INTO buckets (name, description, organization_id, user_id, bucket_key, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *";
                let q = sqlx::query_as::<_, Bucket>(sql)
                    .bind(&item.name)
                    .bind(&item.description)
                    .bind(item.organization_id)
                    .bind(item.user_id)
                    .bind(bucket_key)
                    .bind(Local::now())
                    .bind(Local::now());
                let bucket = q.fetch_one(pool).await?;
                println!("done creating bucket {:?}", bucket);
                Ok(bucket)
            }
            _ => todo!("auth_create_bucket on sqlite"),
        }
    }

    pub async fn list(db: DbStorage, organization_id: i64) -> Result<Vec<Bucket>> {
        let pool = db.pool.clone();
        let sql = "SELECT * FROM buckets where organization_id = $1 ORDER BY id DESC";
        let reports = match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                sqlx::query_as::<_, Bucket>(sql)
                    .bind(organization_id)
                    .fetch_all(pool)
                    .await?
            }
            _ => todo!("auth_buckets_list"),
        };
        Ok(reports)
    }
    pub async fn find_by_apikey(db: DbStorage, apikey: String) -> Result<Bucket> {
        let pool = db.pool.clone();
        let sql = "SELECT * FROM buckets where bucket_key = $1";
        let reports = match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                sqlx::query_as::<_, Bucket>(sql)
                    .bind(apikey)
                    .fetch_one(pool)
                    .await?
            }
            _ => todo!("auth_buckets_list"),
        };
        Ok(reports)
    }
}

#[derive(Debug, Subcommand, Clone, Serialize, Deserialize)]
pub enum BucketCommands {
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

async fn listBuckets(db: DbStorage, pattern: Option<String>) -> Result<()> {
    let pool = db.pool.clone();
    let buckets = match pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let pattern = format!("%{}%", pattern.unwrap_or_default());
            let buckets =
                sqlx::query_as::<_, Bucket>("select * from buckets where name ilike $1")
                    .bind(&pattern)
                    .fetch_all(pool)
                    .await;
            buckets.unwrap()
        }
        DatabasePool::Sqlite(pool) => todo!("listBuckets on sqlite"),
    };
    println!("{}", serde_json::to_string_pretty(&buckets)?);
    Ok(())
}
pub async fn auth_bucket_commands(ctx: Arc<AppContext>, command: BucketCommands) -> Result<()> {
    match command {
        BucketCommands::Add { name } => {
            //auth_create_bucket(ctx.storage.clone(), NewBucket { user_id: 0, name }).await?;
        }
        BucketCommands::Search { pattern } => {
            listBuckets(ctx.storage.clone(), pattern).await;
        }
        BucketCommands::Disable { id } => todo!("bucket disable not implemented"),
    };
    Ok(())
}
