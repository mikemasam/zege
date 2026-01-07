use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use anyhow::Error as AnyhowError;
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;

use crate::utils::appconfig::applogger;
#[derive(Debug, Deserialize, Serialize)]
pub struct DataCursor {
    pub page: i32,
    pub per_page: i32,
}
impl DataCursor {
    pub fn new(page: i32, per_page: i32) -> DataCursor {
        let mut _page = page;
        if (page < 0) {
            _page = 0;
        };
        DataCursor {
            page: _page,
            per_page,
        }
    }
    pub fn offset(&self) -> i64 {
        return (self.page * self.per_page) as i64;
    }
    pub fn limit(&self) -> i64 {
        return self.per_page as i64;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppResponse<T> {
    pub status: i32,
    pub message: String,
    pub data: Option<T>,
    pub cursor: Option<DataCursor>,
}
impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        // Convert the struct itself into a JSON response with status
        (
            StatusCode::from_u16(self.status as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        )
            .into_response()
    }
}

impl<T> AppResponse<T> {
    pub fn created(data: Option<T>, message: Option<&str>) -> Result<Self, AppError> {
        Ok(AppResponse {
            status: 201,
            message: message.unwrap_or("").to_string(),
            data,
            cursor: None,
        })
    }
    pub fn ok(data: Option<T>, message: Option<&str>) -> Result<Self, AppError> {
        Ok(AppResponse {
            status: 200,
            message: message.unwrap_or("").to_string(),
            data,
            cursor: None,
        })
    }
    pub fn cursor(
        data: Option<T>,
        cursor: DataCursor,
        message: Option<&str>,
    ) -> Result<Self, AppError> {
        Ok(AppResponse {
            status: 200,
            message: message.unwrap_or("").to_string(),
            data,
            cursor: Some(cursor),
        })
    }
    pub fn error(message: &str, data: Option<T>) -> Result<Self, AppError> {
        Ok(AppResponse {
            status: 400,
            message: message.to_string(),
            data,
            cursor: None,
        })
    }
    pub fn unauthorized(message: &str) -> Result<Self, AppError> {
        Ok(AppResponse {
            status: 401,
            message: message.to_string(),
            data: None,
            cursor: None,
        })
    }
}

#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError(e)
    }
}

impl From<SqlxError> for AppError {
    fn from(e: SqlxError) -> Self {
        // wrap the sqlx error into anyhow first
        AppError(AnyhowError::new(e))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        applogger::error(format!("error occured {:?}", self));
        AppResponse::<()>::error(self.0.to_string().as_str(), None).into_response()
    }
}
pub type AppResult<T> = Result<AppResponse<T>, AppError>;

#[macro_export]
macro_rules! api_ensure {
    ($cond:expr, $msg:expr) => {
        if !$cond {
            return Err(anyhow::Error::msg($msg).into());
        }
    };
}
