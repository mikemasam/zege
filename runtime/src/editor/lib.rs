
use serde::{Deserialize, Serialize};

use crate::db::lib::QueryResult;

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse {
    pub data: Option<QueryResult>,
    pub error: Option<String>,
    pub status: i32,
}

#[derive(Deserialize)]
pub struct InputData {
    pub sql: String,
    pub connection_id: String,
}



