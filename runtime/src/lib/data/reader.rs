use anyhow::Result;
use serde_json::Value;

use crate::{
    ctx::{appcontext::DbStorage, dbmanager::DatabasePool},
    utils::dbutil::StreamJsonExt,
};

pub struct DataReader {}

impl DataReader {
    pub async fn read(storage: DbStorage, organization_id: i64, sql: String) -> Result<Vec<Value>> {
        match storage.pool.as_ref().unwrap() {
            DatabasePool::Postgres(pool) => {
                let mut tx = pool.begin().await?;
                sqlx::query(format!("SET LOCAL app.organization_id = {organization_id}").as_str())
                    .execute(tx.as_mut())
                    .await?;
                sqlx::query("SET LOCAL ROLE zege_events_read_user")
                    .execute(tx.as_mut())
                    .await?;
                let rows = sqlx::query(sql.as_str())
                    .fetch(tx.as_mut())
                    .json(100)
                    .await?;

                tx.commit().await?;
                Ok(rows)
            }
            _ => todo!("report_read_route"),
        }
    }
}
