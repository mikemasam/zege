use futures::{Stream, StreamExt};
use serde_json::{Map, Value, json};
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Error as SqlxError, Row, TypeInfo, ValueRef};
use std::collections::HashMap;

fn get_column_info(row: &SqliteRow, idx: usize) -> (String, Value, &str) {
    let col_name = row.column(idx).name().to_owned();
    macro_rules! try_get {
        ($t:ty) => {
            if let Ok(v) = row.try_get::<$t, _>(idx) {
                return (col_name, json!(v), stringify!($t));
            }
        };
    }
    try_get!(i64);
    try_get!(f64);
    try_get!(bool);
    try_get!(String);
    try_get!(&str);
    try_get!(Option<i64>);
    try_get!(Option<f64>);
    try_get!(Option<String>);
    try_get!(Option<&str>);
    (col_name, json!(null), "nan") // BLOB, etc.
}

pub async fn rows_to_json_vec(
    mut stream: impl Stream<Item = Result<SqliteRow, SqlxError>> + Unpin,
) -> Result<Vec<Value>, SqlxError> {
    let mut results = Vec::new();
    while let Some(row_result) = stream.next().await {
        let row = row_result?; // Propagates Err if any
        let columns = row.columns();
        let mut map = HashMap::with_capacity(columns.len());
        for i in 0..columns.len() {
            let (name, value, _type) = get_column_info(&row, i);
            println!("{name}: {_type}");
            map.insert(name, value);
        }
        results.push(Value::Object(Map::from_iter(map)));
    }
    Ok(results)
}
