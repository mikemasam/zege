use futures::{Stream, StreamExt};
use serde_json::{Map, Value, json};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error as SqlxError, Column, Row, TypeInfo, ValueRef};
use std::collections::HashMap;
fn get_column_info(row: &SqliteRow, idx: usize) -> (String, Value) {
    let col = row.columns().get(idx).unwrap();
    let col_name = col.name().to_string();
    let type_name = col.type_info().name().to_string();

    let val = match type_name.as_str() {
        "INT8" | "INTEGER" | "BIGINT" => row.try_get::<i64, usize>(idx).map(|v| json!(v)),
        "INT4" => row.try_get::<i32, usize>(idx).map(|v| json!(v)),
        "FLOAT4" | "FLOAT8" | "NUMERIC" | "DECIMAL" => {
            row.try_get::<f64, usize>(idx).map(|v| json!(v))
        }
        "BOOL" => row.try_get::<bool, usize>(idx).map(|v| json!(v)),
        "VARCHAR" | "TEXT" | "CHAR" => row.try_get::<String, usize>(idx).map(|v| json!(v)),
        "TIMESTAMP" | "TIMESTAMPTZ" => row.try_get::<String, usize>(idx).map(|v| json!(v)),
        _ => Ok(json!(null)),
    }
    .unwrap_or(json!(null));
    (col_name, val)
}

pub async fn rows_to_json_vec(
    mut stream: impl Stream<Item = Result<SqliteRow, SqlxError>> + Unpin,
) -> Result<Vec<Value>, SqlxError> {
    let mut results = Vec::new();
    while let Some(row_result) = stream.next().await {
        let row = row_result?;  // Propagates Err if any
        let columns = row.columns();
        let mut map = HashMap::with_capacity(columns.len());
        for i in 0..columns.len() {
            let (name, value) = get_column_info(&row, i);
            map.insert(name, value);
        }
        results.push(Value::Object(Map::from_iter(map)));
    }
    Ok(results)
}
