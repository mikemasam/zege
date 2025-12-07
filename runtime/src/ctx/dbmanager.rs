#![allow(dead_code, unused_imports, unused_variables)]
use std::fmt::{Debug, format};
use std::path::Path;
use std::time::Duration;
use std::{default, env};

use sqlx::any::{AnyConnectOptions, AnyPoolOptions};
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{AnyPool, ConnectOptions, Error, PgPool, SqlitePool};

use crate::ctx::appcontext::AppEnv;

#[derive(Debug, Clone)]
pub enum DatabasePool {
    Sqlite(SqlitePool),
    Postgres(PgPool),
}

#[derive(Debug, Clone)]
pub struct DbManager {
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
impl DbManager {
    pub fn is_connected(&self) -> bool {
        self.pool.is_some()
    }
    pub async fn connect(opts: DbManagerConnectOptions) -> Result<Self, DbManagerError> {
        let db_driver = env::var("DB_DRIVER").unwrap_or("sqlite".to_string());
        AppEnv::log(format!("> DB Connecting to {}", db_driver));
        if db_driver.eq_ignore_ascii_case("sqlite") {
            DbManager::connect_sqlite(opts).await
        } else if db_driver.eq_ignore_ascii_case("pgsql") {
            DbManager::connect_pgsql(opts).await
        } else {
            Err(DbManagerError::UnsupportedDriver(db_driver.to_owned()))
        }
    }
    async fn connect_sqlite(opts: DbManagerConnectOptions) -> Result<Self, DbManagerError> {
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
        Ok(DbManager {
            id: dbname.to_owned(),
            pool: Some(DatabasePool::Sqlite(sqlite_pool)),
        })
    }
    async fn connect_pgsql(opts: DbManagerConnectOptions) -> Result<Self, DbManagerError> {
        let dbname = env::var("DB_NAME").expect("env: expected DB_NAME");
        let dbhost = env::var("DB_HOST").expect("env: expected DB_HOST");
        let dbusername = env::var("DB_USERNAME").expect("env: expected DB_USERNAME");
        let dbpassword = env::var("DB_PASSWORD").expect("env: expected DB_PASSWORD");
        let migration_path = "migrations/pgsql";
        let options = PgConnectOptions::new()
            .database(&dbname)
            .host(&dbhost)
            .username(&dbusername)
            .ssl_mode(sqlx::postgres::PgSslMode::Allow)
            .password(&dbpassword);
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect_with(options.clone())
            .await
            .expect("Pgsql connect failed");
        if opts.migrate {
            let migrator: Migrator = Migrator::new(Path::new(migration_path)).await.unwrap();
            migrator.run(&pool).await?;
        }
        Ok(DbManager {
            id: dbname.to_owned(),
            pool: Some(DatabasePool::Postgres(pool)),
        })
    }
    pub async fn close_db(self) {
        if self.pool.is_some() {
            if let DatabasePool::Sqlite(pool) = self.pool.as_ref().unwrap() {
                pool.close().await;
            } else if let DatabasePool::Postgres(pool) = self.pool.as_ref().unwrap() {
                pool.close().await;
            }
        }
    }
}

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
/*
async fn _migrate(pool: &SqlitePool) {
    sqlx::migrate!("migrations/events")
        .run(pool)
        .await
        .expect("failed to run migrations");
}
*/
