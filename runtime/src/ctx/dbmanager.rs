#![allow(dead_code)]
use std::fmt::Debug;
use std::path::Path;
use std::time::Duration;

use sqlx::SqlitePool;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

#[derive(Debug, Clone)]
pub struct DBConnection {}
#[derive(Debug, Clone)]
pub struct DbManager {
    pub id: String,
    pub pool: Option<SqlitePool>,
}

impl DbManager {
    pub fn is_connected(&self) -> bool {
        self.pool.is_some()
    }
    pub async fn check_table_exists(
        pool: &SqlitePool,
        table_name: &str,
    ) -> Result<bool, sqlx::Error> {
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

    pub async fn connect_to_event_db(
        should_migrate: bool,
    ) -> Result<DbManager, sqlx::migrate::MigrateError> {
        let dbname = "data/events.db";
        DbManager::connect_to_event_db_with_name(dbname, should_migrate).await
    }
    pub async fn connect_to_event_db_with_name(
        dbname: &str,
        should_migrate: bool,
    ) -> Result<DbManager, sqlx::migrate::MigrateError> {
        let migration_path = "migrations/events";
        let options = SqliteConnectOptions::new()
            .filename(dbname)
            .create_if_missing(true)
            .pragma("journal_mode", "WAL")
            .pragma("busy_timeout", "10000")
            .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Full);
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect_with(options)
            .await?;
        if should_migrate {
            let migrator: Migrator = Migrator::new(Path::new(migration_path)).await.unwrap();
            migrator.run(&pool).await?;
        }
        Ok(DbManager {
            id: dbname.to_owned(),
            pool: Some(pool),
        })
    }
    pub async fn close_db(self) {
        if let Some(_p) = self.pool {
            let _ = _p.close().await;
        }
    }
}

async fn _migrate(pool: &SqlitePool) {
    sqlx::migrate!("migrations/events")
        .run(pool)
        .await
        .expect("failed to run migrations");
}
