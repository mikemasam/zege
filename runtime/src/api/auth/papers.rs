use anyhow::ensure;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use axum::http::Request;

use crate::{
    lib::auth::user::{config::ConfigPaper, papers::UserPaper},
    utils::http::{AppResponse, AppResult},
};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Paper {
    pub user: Option<UserPaper>,
    pub config: ConfigPaper,
}

pub async fn papers_please<B>(mut req: Request<B>) -> AppResult<Paper> {
    let userpaper = req.extensions().get::<UserPaper>().map(|p| p.to_owned());
    let paper = Paper {
        user: userpaper,
        config: ConfigPaper::new(),
    };
    AppResponse::ok(Some(paper), None)
}
