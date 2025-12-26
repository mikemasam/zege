use anyhow::ensure;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{Extension, http::HeaderMap};

use crate::{
    lib::auth::user::papers::UserPaper,
    utils::http::{AppResponse, AppResult},
};

pub async fn papers_please(Extension(paper): Extension<UserPaper>) -> AppResult<UserPaper> {
    AppResponse::ok(Some(paper), None)
}
