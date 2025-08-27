use std::sync::Arc;

use axum::{
    Extension, Json, Router, body::Body, http::StatusCode, response::IntoResponse, routing,
};
use serde::Deserialize;
use sqlx::{Row, SqlitePool};
use tokio::sync::Mutex;

use crate::db::lib::DbManager;

#[derive(Deserialize)]
pub struct NewConnectionData {
    pub name: String,
    pub dbtype: String,
    pub dbname: String,
    pub host: String,
    pub username: String,
    pub password: String,
}
pub async fn create_connection(
    Extension(dbman): Extension<Arc<Mutex<DbManager>>>,
    Json(payload): Json<NewConnectionData>,
) -> impl IntoResponse {
    let config_url = "sqlite:/tmp/zege.db";
    let mut db = dbman.lock().await;
    match db.find_connection_by_id("config".to_string()).await {
        Some(_con) => _con,
        None => db
            .create_connection("config".to_string(), "sqlite".to_string(), config_url.to_string())
            .unwrap(),
    };

    //return Json("hi");
    let pool = db.get_sqlite_pool("config".to_string()).await;
    //run_migrations(&pool).await.unwrap();
    let result = sqlx::query(
        "INSERT INTO db_connections (name, dbtype, dbname, host, port, username, password) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&payload.name)
    .bind(&payload.dbtype)
    .bind(&payload.dbname)
    .bind(&payload.host)
    .bind(&payload.username)
    .bind(&payload.password)
    .execute(&pool)
    .await.unwrap();
    if result.rows_affected() > 0 {
        return Json("");
    }
    Json("")
    /*
        let row = sqlx::query("select * from db_connections where connection_name = ?")
            .bind(payload.name)
            .fetch_one(&pool)
            .await.unwrap();
        let name: String = row.get(1);
        println!("name: {name}", );
    */
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
