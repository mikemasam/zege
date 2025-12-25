use anyhow::{Result, ensure};
use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{
    auth::user::User,
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    utils::{appenv::AppLogger, security::Security},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ResetPasswordUserDto {
    pub email: String,
    pub current_password: Option<String>,
    pub new_password: Option<String>,
}
pub async fn auth_reset_user_password(
    app: AppContext,
    mut item: ResetPasswordUserDto,
) -> Result<()> {
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    ensure!(!item.email.is_empty(), "Email is required");
    ensure!(item.email.contains('@'), "invalid email");
    if (item.current_password.is_some()) {
        ensure!(
            !item.current_password.as_ref().unwrap().is_empty(),
            "Password is required"
        );
        ensure!(
            item.current_password.as_ref().unwrap().len() >= 4,
            "Password length minimum is 4 characters"
        );
    }
    if (item.new_password.is_some()) {
        ensure!(
            !item.new_password.as_ref().unwrap().is_empty(),
            "Password is required"
        );
        ensure!(
            item.new_password.as_ref().unwrap().len() >= 4,
            "Password length minimum is 4 characters"
        );
    } else {
        item.new_password = Some("zegeadmin".to_string());
    }
    let res = match db.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            let dup = sqlx::query_as::<_, User>("select * from users where email = $1")
                .bind(&item.email)
                .fetch_optional(pool)
                .await?;
            if (item.current_password.is_some()) {
                //TODO: validate current password if is some
            }
            ensure!(dup.is_some(), "User with email does not exists");
            let sql = "UPDATE users set password_hash = $1, updated_at = $2 where email = $3";
            let password = Security::hash_password(&item.new_password.as_ref().unwrap())?;
            let q = sqlx::query(sql)
                .bind(password)
                .bind(Local::now())
                .bind(&item.email);
            q.execute(pool).await?;
            Ok(())
        }
        DatabasePool::Sqlite(pool) => todo!("sqlite_auth_create_user on sqlite"),
    };
    if res.as_ref().is_ok() {
        AppLogger::log(format!(
            "user with email {} has changed there password to {:?}",
            item.email, item.new_password.unwrap()
        ));
    }
    if res.as_ref().is_err() {
        println!("{:?}", res);
    }
    res
}
