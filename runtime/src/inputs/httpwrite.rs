#![allow(dead_code)]
use crate::ctx::appcontext::AppContext;
use crate::event::event::LogEvent;
use crate::http::AppResponse;
use anyhow::Result;
use axum::response::IntoResponse;
use axum::{Extension};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn event_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    body: axum::body::Bytes, // take raw body
) -> impl IntoResponse {
    let parser: Result<Vec<LogEvent>, serde_json::Error> = serde_json::from_slice(&body);
    let payload = match parser {
        Ok(p) => p,
        Err(err) => {
            eprintln!("failed to parse events body {err}");
            return AppResponse::<Value> {
                status: 400,
                message: format!("JSON parse error: {err}"),
                data: None,
            };
        }
    };
    let mut total: u64 = 0;
    let app = appcontext.lock().await;
    for event in payload {
        if let Err(err) = app.event_writer.send(event) {
            eprintln!("Failed to process event {err}");
            return AppResponse {
                status: 400,
                message: format!("Failed to process event: {err}"),
                data: None,
            };
        };
        total += 1;
    }
    AppResponse::<Value> {
        status: 200,
        message: String::new(),
        data: None,
    }
}
