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
    lib::{
        auth::role::{NewRole, Role},
        buckets::{Bucket, NewBucket},
    },
    utils::appconfig::applogger,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrganizationListItem {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub is_current: Option<i64>,
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
    pub async fn create_bare(db: DbStorage, item: NewOrganization) -> Result<Organization> {
        let pool = db.pool.clone();
        ensure!(!item.name.is_empty(), "Name is required");
        ensure!(item.user_id > 0, "User id is required");
        match pool.as_ref().unwrap() {
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
                applogger::log(format!("done creating organization {:?}", org));
                Ok(org)
            }
            _ => todo!("auth_create_organization on sqlite"),
        }
    }
    pub async fn create(db: DbStorage, item: NewOrganization) -> Result<Organization> {
        let org = Organization::create_bare(
            db.clone(),
            NewOrganization {
                name: item.name,
                user_id: item.user_id,
            },
        )
        .await?;
        let role = Role::create(
            db.clone(),
            NewRole {
                name: "Administrator".to_string(),
                description: "Administrator".to_string(),
                organization_id: org.id,
            },
        )
        .await?;
        OrganizationMembership::create(
            db.clone(),
            NewOrganizationMembership {
                organization_id: org.id,
                user_id: item.user_id,
                role_id: role.id,
            },
        )
        .await?;
        Bucket::create(
            db.clone(),
            NewBucket {
                name: "Default Bucket".to_string(),
                description: "Default Bucket".to_string(),
                organization_id: org.id,
                user_id: org.user_id,
            },
        )
        .await?;
        Ok(org)
    }
}

impl OrganizationMembership {
    pub async fn organization(&self, storage: DbStorage) -> Option<Organization> {
        Organization::find_by_id(storage, self.organization_id).await
    }
    pub async fn role(&self, storage: DbStorage) -> Option<Role> {
        Role::find_by_id(storage, self.role_id).await
    }
    pub async fn current(db: DbStorage, user_id: i64) -> Option<Self> {
        let pool = db.pool.clone();
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let sql =
                    "select * from organization_memberships where is_current = 1 and user_id = $1";
                let m = sqlx::query_as::<_, OrganizationMembership>(sql)
                    .bind(user_id)
                    .fetch_one(pool)
                    .await
                    .ok();
                if (m.is_some()) {
                    return m;
                }

                let default_org_sql =
                    "select r.* from organization_memberships as r where r.user_id = $1";
                let default_org = sqlx::query_as::<_, OrganizationMembership>(default_org_sql)
                    .bind(user_id)
                    .fetch_one(pool)
                    .await;

                if (default_org.is_err()) {
                    applogger::error(format!("failed to find default_org {:?}", default_org));
                    return None;
                }
                let switched = OrganizationMembership::switch(
                    db.clone(),
                    SwitchOrganizationMembership {
                        organization_id: default_org.unwrap().organization_id,
                        user_id: user_id,
                    },
                )
                .await;
                if (switched.is_err()) {
                    applogger::error(format!("failed to switch organization {:?}", switched));
                    println!("switch org failed");
                    return None;
                }
                sqlx::query_as::<_, OrganizationMembership>(sql)
                    .bind(user_id)
                    .fetch_one(pool)
                    .await
                    .ok()
            }
            _ => todo!("auth_find_user_by_email on sqlite"),
        }
    }

    pub async fn switch(db: DbStorage, item: SwitchOrganizationMembership) -> Result<()> {
        let pool = db.pool.clone();
        ensure!(item.user_id > 0, "user id is required");
        ensure!(item.organization_id > 0, "organization id is required");
        println!("writing switch 1 {:?}", item);
        match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                println!("writing switch 2");
                let sql = "UPDATE organization_memberships set is_current = 0, updated_at = $1 where user_id = $2 and is_current = 1";
                let q = sqlx::query(sql).bind(Local::now()).bind(item.user_id);
                let _droped = q.execute(pool).await?;
                println!("dropped {:?}", _droped);
                let sql = "UPDATE organization_memberships set is_current = 1, updated_at = $1 where user_id = $2 and organization_id = $3";
                let q = sqlx::query(sql)
                    .bind(Local::now())
                    .bind(item.user_id)
                    .bind(item.organization_id);
                let _aquired = q.execute(pool).await?;
                println!("aquired {:?}", _aquired);
                Ok(())
            }
            _ => todo!("auth_switch_organization db driver not supported"),
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
                applogger::log(format!(
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
    pub async fn list(db: DbStorage, user_id: Option<i64>) -> Result<Vec<OrganizationListItem>> {
        let pool = db.pool.clone();
        let is_current_sql = "select id from organization_memberships as r where organization_id = org.id and r.user_id = $1 and r.is_current = 1";
        let exists_in = "SELECT 1 FROM organization_memberships AS e WHERE e.user_id = $1 AND e.organization_id = org.id";
        let sql = format!(
            "SELECT org.*, ({is_current_sql}) is_current FROM organizations AS org WHERE EXISTS ({exists_in}) ORDER BY org.id DESC"
        );
        let reports = match pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                sqlx::query_as::<_, OrganizationListItem>(sql.as_str())
                    .bind(user_id)
                    .fetch_all(pool)
                    .await?
            }
            _ => todo!("auth_organizations_list"),
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
    let organizations = Organization::list(storage, None).await?;
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
