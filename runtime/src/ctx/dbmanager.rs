use std::fmt::{Debug, format};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use std::{default, env};

use anyhow::Result;
use sqlx::any::{AnyConnectOptions, AnyPoolOptions};
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::prelude::FromRow;
use sqlx::{AnyPool, ConnectOptions, Error, PgPool};

use crate::appconfig;
use crate::utils::appconfig::applogger;

#[derive(Debug, Clone)]
pub enum DatabasePool {
    Postgres(PgPool),
}

#[derive(Debug)]
pub struct DbPoolManager {
    pub id: String,
    pub pool: Option<DatabasePool>,
}

#[derive(Debug, Clone)]
pub struct DbManagerConnectOptions {
    pub backup: bool,
    pub migrate: bool,
}
#[derive(Debug)]
pub enum DbManagerError {
    UnsupportedDriver(String),
    Sqlx(sqlx::Error),
    Migrate(sqlx::migrate::MigrateError),
}

impl From<sqlx::migrate::MigrateError> for DbManagerError {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        DbManagerError::Migrate(e)
    }
}
impl DbPoolManager {
    pub fn is_connected(&self) -> bool {
        self.pool.is_some()
    }
    pub async fn connect(opts: DbManagerConnectOptions) -> Result<Self, DbManagerError> {
        let db_driver = appconfig!()
            .database
            .driver
            .clone()
            .unwrap_or("pgsql".to_string());
        if db_driver.eq_ignore_ascii_case("pgsql") {
            DbPoolManager::connect_pgsql(opts).await
        } else {
            Err(DbManagerError::UnsupportedDriver(db_driver.to_owned()))
        }
    }
    /*
    async fn connect_sqlite(opts: DbManagerConnectOptions) -> Result<Self, DbManagerError> {
        todo!("db driver not supported");
        let mut dbname = env::var("DB_NAME").unwrap_or("zege.db".to_string());
        if opts.backup {
            dbname = format!("{}.backup", dbname);
        }
        let migration_path = "migrations/sqlite";
        let options = SqliteConnectOptions::new()
            .filename(dbname.clone())
            .create_if_missing(true)
            .pragma("journal_mode", "WAL")
            .pragma("busy_timeout", "10000")
            .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Full);
        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options.clone())
            .await
            .expect("Sqlite connect failed");
        if opts.migrate {
            let migrator: Migrator = Migrator::new(Path::new(migration_path)).await.unwrap();
            migrator.run(&sqlite_pool).await?;
        }
        Ok(DbPoolManager {
            id: dbname.to_owned(),
            pool: Some(DatabasePool::Sqlite(sqlite_pool)),
        })
    }
    */
    async fn connect_pgsql(opts: DbManagerConnectOptions) -> Result<Self, DbManagerError> {
        let db_config = &appconfig!().database;
        applogger::log(format!("DB psql {}", db_config.name.as_str()));
        let migration_path = "migrations/pgsql";
        let options = PgConnectOptions::new()
            .database(db_config.name.as_str())
            .host(db_config.host.as_str())
            .username(db_config.username.as_str())
            .ssl_mode(sqlx::postgres::PgSslMode::Allow)
            .password(db_config.password.as_str());
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect_with(options.clone())
            .await
            .expect("Pgsql connect failed");
        if opts.migrate {
            let migrator: Migrator = Migrator::new(Path::new(migration_path)).await.unwrap();
            migrator.run(&pool).await?;
        }
        Ok(DbPoolManager {
            id: db_config.name.clone(),
            pool: Some(DatabasePool::Postgres(pool)),
        })
    }
    pub async fn close_db(self) {
        if self.pool.is_some() {
            if let DatabasePool::Postgres(pool) = self.pool.as_ref().unwrap() {
                pool.close().await;
            }
        }
    }
}

/*
fn sqlite_insert(_sql: String) {}
pub async fn check_table_exists(pool: &SqlitePool, table_name: &str) -> Result<bool, sqlx::Error> {
    let checker = r#"
            SELECT COUNT(*) FROM information_schema.tables 
            WHERE table_schema = 'public' AND table_name = $1
        "#;
    let row: (i64,) = sqlx::query_as(checker)
        .bind(table_name)
        .fetch_one(pool)
        .await?;
    Ok(row.0 > 0)
}
*/
/*
async fn _migrate(pool: &SqlitePool) {
    sqlx::migrate!("migrations/events")
        .run(pool)
        .await
        .expect("failed to run migrations");
}
*/
