mod auth;
mod events;
mod organizations;
mod report;
mod roles;
mod services;
mod users;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{Router, http::Request, middleware, response::IntoResponse, routing};
use serde_json::Value;

use crate::{
    api::{
        auth::{auth_private_routes, auth_public_routes},
        events::{events_routes, old_event_input_routes},
        organizations::organizations_routes,
        report::report_routes,
        roles::roles_routes,
        services::services_routes,
        users::users_routes,
    }, api_ensure, ctx::appcontext::{self, AppContext}, lib::auth::user::papers::UserPaper, utils::{appenv::AppLogger, http::AppResponse}
};

pub fn api_routes() -> Router {
    let _private = Router::new()
        .nest("/services", services_routes())
        .nest("/roles", roles_routes())
        .nest("/users", users_routes())
        .nest("/organizations", organizations_routes())
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
    let papers_result = UserPaper::verify_token(appcontext.storage.clone(), token).await;
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
