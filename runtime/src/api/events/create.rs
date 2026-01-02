#![allow(dead_code)]
use crate::api_ensure;
use crate::ctx::appcontext::AppContext;
use crate::dto::logevent::{LogEvent, LogEventChannelMessage, LogEventInput};
use crate::utils::http::{AppResponse, AppResult};
use axum::Extension;
use axum::extract::Query;
use axum::response::IntoResponse;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn event_write_route(
    Extension(ctx): Extension<Arc<AppContext>>,
    Query(params): Query<HashMap<String, String>>,
    body: axum::body::Bytes, // take raw body
) -> AppResult<Value> {
    let service_apikey = params.get("apikey");
    api_ensure!(
        service_apikey.is_some(),
        "Failed to write event, api Key is required"
    );
    //println!("{:?}", body);
    let parser: Result<Vec<LogEventInput>, serde_json::Error> = serde_json::from_slice(&body);
    let payload = match parser {
        Ok(p) => p,
        Err(err) => {
            eprintln!("failed to parse events body {err}");
            return AppResponse::error(format!("JSON parse error: {err}").as_str(), None);
        }
    };

    //println!("{}", serde_json::to_string_pretty(&payload).unwrap());
    for mut event in payload {
        event.apikey_value = Some(service_apikey.unwrap().to_string());
        if let Err(err) = ctx
            .event_writer
            .send(LogEventChannelMessage::Data(Box::new(event)))
        {
            eprintln!("Failed to process event {err}");
            return AppResponse::error(format!("Failed to process event: {err}").as_str(), None);
        };
    }
    AppResponse::ok(None, None)
}
