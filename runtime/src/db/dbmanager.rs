#![allow(dead_code)]
use futures::TryStreamExt;
use serde::Serialize;
use serde::Serializer;
use serde_json::Value;
use sqlx::SqlitePool;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;

use sqlx::{Column, Row, TypeInfo, postgres::PgPoolOptions};

use crate::db::lib::DBConnection;
use crate::db::lib::DBPool;
use crate::db::lib::DBPoolType;
use crate::db::lib::DbManager;
use crate::db::lib::QueryResult;

const MAX_RECORDS: i64 = 100;
#[derive(sqlx::FromRow, Debug, Clone, Serialize)]
pub struct CountResult {
    total: i64,
}

#[derive(Debug, Serialize)]
struct ColumnValue {
    index: usize,
    name: String,
    type_name: String,
    value: DbValue,
}

impl Serialize for DbValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DbValue::Int(v) => serializer.serialize_i64(*v),
            DbValue::Int32(v) => serializer.serialize_i32(*v),
            DbValue::Float(v) => serializer.serialize_f64(*v),
            DbValue::Bool(v) => serializer.serialize_bool(*v),
            DbValue::String(v) => serializer.serialize_str(v),
            DbValue::Timestamp(v) => serializer.serialize_str(v),
            DbValue::Unsupported | DbValue::Null => serializer.serialize_none(),
        }
    }
}
#[derive(Debug)]
enum DbValue {
    Int(i64),
    Int32(i32),
    Float(f64),
    Bool(bool),
    String(String),
    Timestamp(String),
    Unsupported,
    Null,
}

fn get_column_info(row: &PgRow, idx: usize) -> ColumnValue {
    let col = row.columns().get(idx).unwrap();
    let col_name = col.name().to_string();
    let type_name = col.type_info().name().to_string();

    let value = match type_name.as_str() {
        "INT8" | "INTEGER" | "BIGINT" => row
            .try_get::<i64, usize>(idx)
            .map(DbValue::Int)
            .unwrap_or(DbValue::Null),
        "INT4" => row
            .try_get::<i32, usize>(idx)
            .map(DbValue::Int32)
            .unwrap_or(DbValue::Null),
        "FLOAT4" | "FLOAT8" | "NUMERIC" | "DECIMAL" => row
            .try_get::<f64, usize>(idx)
            .map(DbValue::Float)
            .unwrap_or(DbValue::Null),
        "BOOL" => row
            .try_get::<bool, usize>(idx)
            .map(DbValue::Bool)
            .unwrap_or(DbValue::Null),
        "VARCHAR" | "TEXT" | "CHAR" => row
            .try_get::<String, usize>(idx)
            .map(DbValue::String)
            .unwrap_or(DbValue::Null),
        "TIMESTAMP" | "TIMESTAMPTZ" => row
            .try_get::<String, usize>(idx)
            .map(DbValue::Timestamp)
            .unwrap_or(DbValue::Null),
        _ => DbValue::Unsupported,
    };

    ColumnValue {
        index: idx,
        name: col_name,
        type_name,
        value,
    }
}

impl DbManager {
    pub fn new() -> DbManager {
        DbManager {
            connections: vec![],
        }
    }

    pub fn create_connection(
        &mut self,
        id: String,
        _dbtype: String,
        url: String,
    ) -> Option<Arc<Mutex<DBConnection>>> {
        let dbtype: DBPoolType = match _dbtype.as_str() {
            "postgres" => DBPoolType::PostgresCon(),
            "sqlite" => DBPoolType::SqliteCon(),
            _ => DBPoolType::UnknownCon(),
        };
        let d = DBConnection {
            dbtype,
            url,
            id,
            pool: None,
        };
        let con = Arc::new(Mutex::new(d));
        self.connections.push(con.clone());
        Some(con)
    }
    pub fn add_sample_connection(&mut self) -> Option<Arc<Mutex<DBConnection>>> {
        let d = DBConnection {
            url: env::var("DEV_CONNECTION").unwrap(),
            id: String::from("sample"),
            pool: None,
            dbtype: DBPoolType::PostgresCon(),
        };
        let con = Arc::new(Mutex::new(d));
        self.connections.push(con.clone());
        Some(con)
    }
    pub async fn find_connection_by_id(&self, id: String) -> Option<Arc<Mutex<DBConnection>>> {
        for conn in &self.connections {
            // Lock the mutex to access DBConnection safely
            if let Ok(db_conn) = conn.try_lock() {
                if db_conn.id == id {
                    // Found it; clone Arc to return
                    return Some(Arc::clone(conn));
                }
            } else {
                // If lock poisoned or failed, just skip
                continue;
            }
        }
        None
    }
    async fn get_connection(
        &mut self,
        id: String,
    ) -> Result<Arc<Mutex<DBConnection>>, sqlx::Error> {
        let con = self.find_connection_by_id(id).await.unwrap();
        let mut conn = con.lock().await;
        if conn.pool.is_none() {
            println!("connecting.... {}", conn.id);
            match conn.dbtype {
                DBPoolType::PostgresCon() => {
                    println!("connecting to postgres {}", conn.url);
                    let pool = PgPoolOptions::new()
                        .max_connections(5)
                        .connect(conn.url.as_str())
                        .await?;
                    conn.pool = Some(DBPool::PostgresCon(pool));
                }
                DBPoolType::SqliteCon() => {
                    println!("connecting to sqlite {}", conn.url);
                    let pool = SqlitePoolOptions::new()
                        .max_connections(5)
                        .connect(conn.url.as_str())
                        .await?;
                    conn.pool = Some(DBPool::SqliteCon(pool));
                }
                _ => {
                    println!("connect type not supported {}", conn.id);
                }
            };
            println!("connected to {}", conn.id);
        }

        let pool = conn.pool.as_ref().unwrap();
        match pool {
            DBPool::PostgresCon(pg_pool) => {
                if pg_pool.is_closed() {
                    return Err(sqlx::Error::PoolClosed);
                }
                println!("~ postgres pool");
            }
            DBPool::SqliteCon(lite_pool) => {
                if lite_pool.is_closed() {
                    return Err(sqlx::Error::PoolClosed);
                }
                println!("~ pool");
            }
            _ => {
                panic!("Connection type not implemented");
            }
        };
        Ok(con.clone())
    }

    fn executor(&self) {}
    pub async fn exec(
        &mut self,
        connection_id: String,
        sql: String,
    ) -> Result<QueryResult, sqlx::Error> {
        let (results, count) = self.exec_table(connection_id.clone(), sql.clone()).await?;
        Ok(QueryResult {
            connection_id,
            sql,
            data: results,
            count,
        })
    }

    pub async fn get_sqlite_pool(&mut self, connection_id: String) -> sqlx::SqlitePool {
        let con = self
            .get_connection(connection_id)
            .await
            .expect("Connection must exist");
        let con_guard = con.lock().await;

        match con_guard.pool.as_ref().unwrap() {
            DBPool::SqliteCon(pg_pool) => pg_pool.clone(),
            _ => panic!("Not a Sqlite connection"),
        }
    }
    pub async fn get_postgres_pool(&mut self, connection_id: String) -> sqlx::PgPool {
        let con = self
            .get_connection(connection_id)
            .await
            .expect("Connection must exist");
        let con_guard = con.lock().await;

        match con_guard.pool.as_ref().unwrap() {
            DBPool::PostgresCon(pg_pool) => pg_pool.clone(),
            _ => panic!("Not a Postgres connection"),
        }
    }
    async fn exec_count(
        &mut self,
        connection_id: String,
        sql: String,
    ) -> Result<CountResult, sqlx::Error> {
        let _con = self
            .get_connection(connection_id)
            .await
            .expect("Connection must exist");
        let con_guard = _con.lock().await;
        let pool = match con_guard.pool.as_ref().unwrap() {
            DBPool::PostgresCon(pg_pool) => pg_pool,
            _ => panic!("Not a Postgres connection"),
        };
        let count_sql = format!("SELECT COUNT(*) as total FROM ({}) as tb", sql.as_str());
        println!("{}", count_sql.as_str());
        let row: CountResult = sqlx::query_as::<_, CountResult>(count_sql.as_str())
            .fetch_one(pool)
            .await?;
        Ok(row)
    }
    async fn exec_table(
        &mut self,
        connection_id: String,
        sql: String,
    ) -> Result<(Vec<Value>, i64), sqlx::Error> {
        let _con = self.get_connection(connection_id).await?;
        let con_guard = _con.lock().await;
        let pool = match con_guard.pool.as_ref().unwrap() {
            DBPool::PostgresCon(pg_pool) => pg_pool,
            _ => return Err(sqlx::Error::Protocol("Not a Postgres connection".into())),
        };
        let mut results = Vec::new();
        let mut rows = sqlx::query(sql.as_str()).fetch(pool);
        let mut l: i64 = 0;
        while let Some(row) = rows.try_next().await? {
            let mut row_info = Vec::new();
            let col_count = row.columns().len();
            for i in 0..col_count {
                //let col = row.columns().get(i).unwrap();
                let col_info = get_column_info(&row, i);
                row_info.push(serde_json::to_value(col_info).unwrap());
            }
            results.push(Value::Array(row_info));
            l += 1;
            if l >= MAX_RECORDS {
                break;
            }
        }
        Ok((results, l))
    }
}
