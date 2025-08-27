use std::sync::Arc;
use serde::Serialize;
use serde_json::Value;
use sqlx::{MySql, Pool, Postgres, Sqlite};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct DBConnection {
    pub id: String,
    pub url: String,
    pub dbtype: DBPoolType,
    pub pool: Option<DBPool>,
}
#[derive(Debug, Clone)]
pub enum DBPool {
    PostgresCon(Pool<Postgres>),
    MySqlCon(Pool<MySql>),
    SqliteCon(Pool<Sqlite>),
}
#[derive(Debug, Clone)]
pub enum DBPoolType {
    PostgresCon(),
    MySqlCon(),
    SqliteCon(),
    UnknownCon(),
}
#[derive(Debug, Clone)]
pub struct DbManager {
    pub connections: Vec<Arc<Mutex<DBConnection>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryResult {
    pub connection_id: String,
    pub sql: String,
    pub data: Vec<Value>,
    pub count: i64,
}
