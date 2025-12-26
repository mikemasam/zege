mod auth;
mod events;
mod report;
mod roles;
mod teams;
mod users;
mod services;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{Router, http::Request, middleware, response::IntoResponse, routing};
use serde_json::Value;

use crate::{
    api::{
        auth::{auth_private_routes, auth_public_routes},
        events::{events_routes, old_event_input_routes},
        report::report_routes,
        roles::roles_routes,
        teams::teams_routes,
        users::users_routes,
    },
    api_ensure,
    auth::user::papers::auth_login_verify_user_token,
    ctx::appcontext::{self, AppContext},
    utils::{appenv::AppLogger, http::AppResponse},
};

pub fn api_routes() -> Router {
    let _private = Router::new()
        .nest("/roles", roles_routes())
        .nest("/users", users_routes())
        .nest("/teams", teams_routes())
        .nest("/events", events_routes())
        .nest("/reports", report_routes())
        .nest("/auth", auth_private_routes())
        .layer(middleware::from_fn(auth_middleware));
    Router::new()
        .merge(old_event_input_routes())
        .merge(auth_public_routes())
        .merge(_private)
}

async fn auth_middleware<B>(
    mut req: Request<B>,
    next: middleware::Next<B>,
) -> impl axum::response::IntoResponse {
    let appcontext = match req.extensions().get::<Arc<AppContext>>() {
        Some(ctx) => ctx.clone(),
        None => {
            AppLogger::debug(format!(
                "Unauthorized {} {}",
                req.method(),
                req.uri().path()
            ));
            return AppResponse::<Value>::unauthorized("App context missing").into_response();
        }
    };
    let token = match req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
    {
        Some(t) => t,
        None => {
            AppLogger::debug(format!(
                "Unauthorized {} {}",
                req.method(),
                req.uri().path()
            ));
            return AppResponse::<Value>::unauthorized("Unauthorized").into_response();
        }
    };
    let papers_result = auth_login_verify_user_token(appcontext.clone(), token).await;
    if (papers_result.is_err()) {
        AppLogger::debug(format!(
            "Unauthorized {} {}",
            req.method(),
            req.uri().path()
        ));
        return AppResponse::<Value>::unauthorized("Unauthorized").into_response();
    }
    req.extensions_mut().insert(papers_result.unwrap());
    AppLogger::debug(format!("Authorized {} {}", req.method(), req.uri().path()));
    next.run(req).await
}
