mod auth;
mod buckets;
mod data;
mod events;
mod organizations;
mod report;
mod roles;
mod users;
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{Router, http::Request, middleware, response::IntoResponse, routing};
use serde_json::Value;

use crate::{
    api::{
        auth::{auth_private_routes, auth_public_routes},
        buckets::buckets_routes,
        data::data_routes,
        events::{events_routes, old_event_input_routes},
        organizations::organizations_routes,
        report::report_routes,
        roles::roles_routes,
        users::users_routes,
    },
    api_ensure,
    ctx::appcontext::{self, AppContext},
    lib::auth::user::papers::UserPaper,
    utils::{http::AppResponse, logging::AppLogger},
};

pub fn api_routes() -> Router {
    let _private = Router::new()
        .nest("/buckets", buckets_routes())
        .nest("/roles", roles_routes())
        .nest("/users", users_routes())
        .nest("/organizations", organizations_routes())
        .nest("/events", events_routes())
        .nest("/reports", report_routes())
        .nest("/data", data_routes())
        .nest("/auth", auth_private_routes())
        .layer(middleware::from_fn(private_middleware));
    let _public = Router::new()
        .merge(old_event_input_routes())
        .nest("/auth", auth_public_routes())
        .layer(middleware::from_fn(public_middleware));
    Router::new().merge(_private).merge(_public)
}

async fn private_middleware<B>(
    mut req: Request<B>,
    next: middleware::Next<B>,
) -> impl axum::response::IntoResponse {
    let papers_result = get_verified_user(&req).await;
    if let Ok(paper) = papers_result {
        req.extensions_mut().insert(paper);
    } else {
        AppLogger::debug(format!(
            "Unauthorized {} {}",
            req.method(),
            req.uri().path()
        ));
        return AppResponse::<Value>::unauthorized("Unauthorized").into_response();
    }
    //AppLogger::debug(format!("Authorized {} {}", req.method(), req.uri().path()));
    next.run(req).await
}

async fn public_middleware<B>(
    mut req: Request<B>,
    next: middleware::Next<B>,
) -> impl axum::response::IntoResponse {
    let papers_result = get_verified_user(&req).await;
    if let Ok(paper) = papers_result {
        req.extensions_mut().insert(paper);
    }
    next.run(req).await
}

async fn get_verified_user<B>(req: &Request<B>) -> Result<UserPaper> {
    let appcontext = req
        .extensions()
        .get::<Arc<AppContext>>()
        .cloned()
        .context("Context failed to load")?;
    let token = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .context("authorization header not set")?;
    UserPaper::verify_token(appcontext.storage.clone(), token).await
}
