use std::sync::Arc;

use anyhow::{Result, ensure};
use chrono::{DateTime, FixedOffset, Local};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    ctx::{
        appcontext::{AppContext, DbStorage},
        dbmanager::DatabasePool,
    },
    lib::auth::role::Role,
    utils::appenv::AppLogger,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewOrganization {
    pub name: String,
    pub user_id: i64,
}
#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct OrganizationMembership {
    pub id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub role_id: i64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct NewOrganizationMembership {
    pub organization_id: i64,
    pub user_id: i64,
    pub role_id: i64,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SwitchOrganizationMembership {
    pub organization_id: i64,
    pub user_id: i64,
}
impl Organization {
    pub async fn create(db: DbStorage, item: NewOrganization) -> Result<Organization> {
        let pool = db.pool.clone();
        ensure!(!item.name.is_empty(), "Name is required");
        ensure!(item.user_id > 0, "User id is required");
        match pool.as_ref().unwrap() {
            DatabasePool::Sqlite(pool) => todo!("auth_create_organization on sqlite"),
            DatabasePool::Postgres(pool) => {
                let dup = sqlx::query_as::<_, Organization>(
                    "select * from organizations where name = $1 and user_id = $2",
                )
                .bind(&item.name)
                .bind(item.user_id)
                .fetch_optional(pool)
                .await?;
                ensure!(dup.is_none(), "Organization with name already exists");
                let sql = "INSERT INTO organizations (name, user_id, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *";
                let q = sqlx::query_as::<_, Organization>(sql)
                    .bind(&item.name)
                    .bind(item.user_id)
                    .bind(Local::now())
                    .bind(Local::now());
                let org = q.fetch_one(pool).await?;
                AppLogger::log(format!("done creating organization {:?}", org));
                Ok(org)
            }
        }
    }
}

impl OrganizationMembership {
    pub async fn organization(&self, storage: DbStorage) -> Option<Organization> {
        Organization::find_by_id(storage, self.organization_id).await
    }
    pub async fn role(&self, storage: DbStorage) -> Option<Role> {
        Role::find_by_id(storage, self.role_id).await
    }
    pub async fn current(storage: DbStorage, user_id: i64) -> Option<Self> {
        let pool = storage.pool.clone();
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let m = sqlx::query_as::<_, OrganizationMembership>(
                    "select * from organization_memberships where is_current = 1 and user_id = $1",
                )
                .bind(user_id)
                .fetch_one(pool)
                .await;
                m.ok()
            }
            _ => todo!("auth_find_user_by_email on sqlite"),
        }
    }

    pub async fn create(
        db: DbStorage,
        item: NewOrganizationMembership,
    ) -> Result<OrganizationMembership> {
        let pool = db.pool.clone();
        ensure!(item.user_id > 0, "user id is required");
        ensure!(item.organization_id > 0, "organization id is required");
        ensure!(item.role_id > 0, "role id is required");
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let dup = sqlx::query_as::<_, OrganizationMembership>(
                "select * from organization_memberships where organization_id = $1 and user_id = $2",
            )
            .bind(item.organization_id)
            .bind(item.user_id)
            .fetch_optional(pool)
            .await?;
                ensure!(dup.is_none(), "organization_memberships already exists");
                let sql = "INSERT INTO organization_memberships (organization_id, user_id, role_id, is_current, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *";
                let q = sqlx::query_as::<_, OrganizationMembership>(sql)
                    .bind(item.organization_id)
                    .bind(item.user_id)
                    .bind(item.role_id)
                    .bind(0)
                    .bind(Local::now())
                    .bind(Local::now());
                let organization_membership = q.fetch_one(pool).await?;
                AppLogger::log(format!(
                    "done creating organization_memberships {:?}",
                    organization_membership
                ));
                Ok(organization_membership)
            }
            _ => todo!("auth_create_organization_membership db driver not supported"),
        }
    }
}

impl Organization {
    pub async fn find_by_id(storage: DbStorage, id: i64) -> Option<Organization> {
        let pool = storage.pool.clone();
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let _org =
                    sqlx::query_as::<_, Organization>("select * from organizations where id = $1")
                        .bind(id)
                        .fetch_one(pool)
                        .await;
                Some(_org.unwrap())
            }
            _ => todo!("auth_find_user_by_email on sqlite"),
        }
    }
    pub async fn switch(db: DbStorage, item: SwitchOrganizationMembership) -> Result<()> {
        let pool = db.pool.clone();
        ensure!(item.user_id > 0, "user id is required");
        ensure!(item.organization_id > 0, "organization id is required");
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let sql = "UPDATE organization_memberships set is_current = 0, updated_at = $1 where user_id = $2 and is_current = 1";
                let q = sqlx::query(sql).bind(Local::now()).bind(item.user_id);
                q.execute(pool).await?;
                let sql = "UPDATE organization_memberships set is_current = 1, updated_at = $1 where user_id = $2 and organization_id = $3";
                let q = sqlx::query(sql)
                    .bind(Local::now())
                    .bind(item.user_id)
                    .bind(item.organization_id);
                q.execute(pool).await?;
                Ok(())
            }
            _ => todo!("auth_switch_organization db driver not supported"),
        }
    }
    pub async fn list(db: DbStorage) -> Result<Vec<Organization>> {
        let pool = db.pool.clone();
        let sql = "SELECT * FROM organizations ORDER BY id DESC";
        let reports = match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                sqlx::query_as::<_, Organization>(sql)
                    .fetch_all(pool)
                    .await?
            }
            DatabasePool::Sqlite(pool) => todo!("auth_organizations_list"),
        };
        Ok(reports)
    }
}

#[derive(Debug, Subcommand, Clone, Serialize, Deserialize)]
pub enum OrganizationCommands {
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

async fn listOrganizations(storage: DbStorage, pattern: Option<String>) -> Result<()> {
    let organizations = Organization::list(storage).await?;
    println!("{}", serde_json::to_string_pretty(&organizations)?);
    Ok(())
}
pub async fn auth_organization_commands(
    ctx: Arc<AppContext>,
    command: OrganizationCommands,
) -> Result<()> {
    match command {
        OrganizationCommands::Add { name } => {
            Organization::create(ctx.storage.clone(), NewOrganization { user_id: 0, name }).await?;
        }
        OrganizationCommands::Search { pattern } => {
            listOrganizations(ctx.storage.clone(), pattern).await;
        }
        OrganizationCommands::Disable { id } => todo!("organization disable not implemented"),
    };
    Ok(())
}
