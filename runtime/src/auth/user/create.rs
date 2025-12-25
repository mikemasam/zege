use anyhow::{ensure, Result};
use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{auth::user::papers::User, ctx::{appcontext::AppContext, dbmanager::DatabasePool}, utils::{appenv::AppLogger, security::Security}};


#[derive(Deserialize, Serialize, Debug)]
pub struct NewUser {
   pub name: String,
   pub email: String,
   pub password: String,
}
pub async fn auth_create_user(app: AppContext, item: NewUser) -> Result<User> {
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    ensure!(!item.name.is_empty(), "Name is required");
    ensure!(!item.email.is_empty(), "Email is required");
    ensure!(item.email.contains('@'), "invalid email");
    ensure!(!item.password.is_empty(), "Password is required");
    ensure!(
        item.password.len() >= 4,
        "Password length minimum is 4 characters"
    );
    let res = match db.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let dup = sqlx::query_as::<_, User>("select * from users where email = $1")
                .bind(&item.email)
                .fetch_optional(pool)
                .await?;
            //println!("{:?}", dup);
            ensure!(
                dup.is_none(),
                "User with email already exists with password 6767"
            );
            let sql = "INSERT INTO users (email, name, password_hash, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *";
            let password = Security::hash_password(&item.password)?;
            let q = sqlx::query_as::<_, User>(sql)
                .bind(&item.email)
                .bind(&item.name)
                .bind(password)
                .bind(Local::now())
                .bind(Local::now());
            let user = q.fetch_one(pool).await?;
            Ok(user)
        }
        DatabasePool::Sqlite(pool) => todo!("sqlite_auth_create_user on sqlite"),
    };
    if res.as_ref().is_ok() {
        AppLogger::log(format!(
            "new user added with id: {}",
            res.as_ref().unwrap().id
        ));
    }
    if res.as_ref().is_err() {
        println!("{:?}", res);
    }
    res
}
