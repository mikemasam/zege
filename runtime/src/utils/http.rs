use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct AppResponse<T> {
    pub status: i32,
    pub message: String,
    pub data: Option<T>,
}
impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        // Convert the struct itself into a JSON response with status
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl<T> AppResponse<T> {
    pub fn created(data: Option<T>, message: Option<&str>) -> Json<Self> {
        Json(AppResponse {
            status: 201,
            message: message.unwrap_or("").to_string(),
            data,
        })
    }
    pub fn ok(data: Option<T>, message: Option<&str>) -> Json<Self> {
        Json(AppResponse {
            status: 200,
            message: message.unwrap_or("").to_string(),
            data,
        })
    }
    pub fn error(message: &str, data: Option<T>) -> Json<Self> {
        Json(AppResponse {
            status: 400,
            message: message.to_string(),
            data,
        })
    }
}
