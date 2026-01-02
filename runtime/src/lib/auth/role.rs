use anyhow::{Result, ensure};
use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Sqlite, prelude::FromRow};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    ctx::{
        appcontext::{AppContext, DbStorage},
        dbmanager::DatabasePool,
    },
    lib::organization,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub organization_id: i64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewRole {
    pub name: String,
    pub description: String,
    pub organization_id: i64,
}
impl Role {
    pub async fn create(db: DbStorage, item: NewRole) -> Result<Role> {
        let pool = db.pool.clone();
        ensure!(!item.name.is_empty(), "Name is required");
        ensure!(item.organization_id > 0, "organization_id is required");
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let dup = sqlx::query_as::<_, Role>(
                    "select * from roles where name = $1 and organization_id = $2",
                )
                .bind(&item.name)
                .bind(item.organization_id)
                .fetch_optional(pool)
                .await?;
                println!("{:?}", dup);
                ensure!(dup.is_none(), "Role with name already exists");
                let sql = "INSERT INTO roles (name, description, organization_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *";
                let q = sqlx::query_as::<_, Role>(sql)
                    .bind(&item.name)
                    .bind(&item.description)
                    .bind(item.organization_id)
                    .bind(Local::now())
                    .bind(Local::now());
                let role = q.fetch_one(pool).await?;
                println!("done creating role {:?}", role);
                Ok(role)
            }
            DatabasePool::Sqlite(pool) => todo!("auth_create_role on sqlite"),
        }
    }
}

pub async fn auth_roles_list(db: DbStorage, organization_id: i64) -> Result<Vec<Role>> {
    let pool = db.pool.clone();
    match pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let sql = "SELECT * FROM roles WHERE organization_id = $1 ORDER BY id DESC";
            let roles = sqlx::query_as::<_, Role>(sql)
                .bind(organization_id)
                .fetch_all(pool)
                .await?;
            Ok(roles)
        }
        _ => todo!("auth_roles_list on sqlite"),
    }
}

impl Role {
    pub async fn find_by_id(storage: DbStorage, id: i64) -> Option<Role> {
        let pool = storage.pool.clone();
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let _org = sqlx::query_as::<_, Role>("select * from roles where id = $1")
                    .bind(id)
                    .fetch_one(pool)
                    .await;
                Some(_org.unwrap())
            }
            _ => todo!("auth_find_role_by_id on db driver"),
        }
    }
}
