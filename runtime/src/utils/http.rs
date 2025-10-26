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
