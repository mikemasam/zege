use std::sync::Arc;
use std::{any::Any, env};

use anyhow::{Result, ensure};
use chrono::{DateTime, Duration, FixedOffset, Local};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::lib::auth::role::Role;
use crate::lib::auth::user::user::UserAccount;
use crate::lib::organization::Organization;
use crate::{
    ctx::{
        appcontext::{AppContext, DbStorage},
        dbmanager::{DatabasePool, DbPoolManager},
    },
    lib::organization::OrganizationMembership,
    utils::security::Security,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPaperRole {
    pub id: i64,
    pub name: String,
}
impl UserPaperRole {
    pub fn make(role: Option<Role>) -> Option<UserPaperRole> {
        match role {
            Some(r) => Some(UserPaperRole {
                id: r.id,
                name: r.name,
            }),
            None => None,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPaperOrganization {
    pub id: i64,
    pub name: String,
}
impl UserPaperOrganization {
    pub fn make(org: Option<Organization>) -> Option<UserPaperOrganization> {
        match org {
            Some(o) => Some(UserPaperOrganization {
                id: o.id,
                name: o.name,
            }),
            None => None,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPaper {
    pub id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub organization: Option<UserPaperOrganization>,
    pub role: Option<UserPaperRole>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    token: String,
    user: UserPaper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub user_id: i64,
    pub exp: usize,
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginCredentials {
    email: String,
    password: String,
}
impl UserPaper {
    pub async fn login_paper(storage: DbStorage, user: &UserAccount) -> Result<LoginResult> {
        Ok(LoginResult {
            token: UserPaper::generate_jwt(user)?,
            user: UserPaper::new(storage, user).await?,
        })
    }
    pub async fn new(storage: DbStorage, user: &UserAccount) -> Result<UserPaper> {
        let mut paper = UserPaper {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            organization: None,
            role: None,
        };

        let membership = OrganizationMembership::current(storage.clone(), user.id).await;
        if let Some(member) = membership {
            paper.organization =
                UserPaperOrganization::make(member.organization(storage.clone()).await);
            paper.role = UserPaperRole::make(member.role(storage.clone()).await);
        }
        Ok(paper)
    }
    pub fn generate_jwt(user: &UserAccount) -> Result<String> {
        let secret = env::var("JWT_SECRET")?;
        let exp = Local::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = AuthClaims {
            sub: Uuid::now_v7().simple().to_string(),
            user_id: user.id,
            email: user.email.as_ref().unwrap().to_string(),
            name: user.name.as_ref().unwrap().to_string(),
            exp,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;
        Ok(token)
    }
    pub fn verify_jwt(token: &str) -> Result<AuthClaims> {
        let secret = std::env::var("JWT_SECRET")?;
        let mut validation = Validation::default();
        validation.validate_exp = true;
        let token_data = decode::<AuthClaims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )?;
        Ok(token_data.claims)
    }
    pub async fn verify_creds(storage: DbStorage, creds: LoginCredentials) -> Result<UserAccount> {
        let email = creds.email;
        let rs_user = UserAccount::find_by_email(storage.clone(), email.clone()).await;
        ensure!(rs_user.is_ok(), "Invalid username or password");
        let user = rs_user.unwrap();
        let valid = Security::verify_password(&creds.password, &user.password_hash);
        ensure!(valid.is_ok(), "Invalid username or password");
        Ok(user)
    }

    pub async fn verify_token(storage: DbStorage, token: &str) -> Result<UserPaper> {
        let claims = UserPaper::verify_jwt(token.replace("Bearer ", "").as_str())?;
        let rs_user = UserAccount::find_by_id(storage.clone(), claims.user_id).await;
        ensure!(rs_user.is_ok(), "unauthorized");
        let user = rs_user.unwrap();
        UserPaper::new(storage.clone(), &user).await
    }
}
