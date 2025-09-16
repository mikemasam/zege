#![allow(dead_code)]
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;

use sqlx::migrate::Migrator;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite, SqlitePool};

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

    pub async fn new(
        id: &str,
        migration_path: Option<&str>,
    ) -> Result<DbManager, sqlx::migrate::MigrateError> {
        let dbname = id.clone();
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(dbname)
            .await?;
        if let Some(_path) = migration_path {
            let migrator: Migrator = Migrator::new(Path::new(_path)).await.unwrap();
            migrator.run(&pool).await?;
        }
        Ok(DbManager {
            id: id.to_owned(),
            pool: Some(pool),
        })
    }
}

async fn _migrate(pool: &SqlitePool) {
    sqlx::migrate!("migrations/events")
        .run(pool)
        .await
        .expect("failed to run migrations");
}
