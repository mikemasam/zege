use futures::{Stream, StreamExt};
use serde_json::{Map, Value, json};
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Error as SqlxError, Row, TypeInfo, ValueRef, postgres::PgRow};

pub trait JsonRow: Row
where
    usize: sqlx::ColumnIndex<Self>,
{
    fn col_count(&self) -> usize {
        self.columns().len()
    }

    fn col_name(&self, idx: usize) -> &str {
        self.column(idx).name()
    }

    fn get_json_value(&self, idx: usize) -> Value;
}
impl JsonRow for PgRow {
    fn get_json_value(&self, idx: usize) -> Value {
        macro_rules! try_get {
            ($t:ty) => {
                if let Ok(v) = self.try_get::<$t, _>(idx) {
                    return json!(v);
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

        Value::Null
    }
}

impl JsonRow for SqliteRow {
    fn get_json_value(&self, idx: usize) -> Value {
        macro_rules! try_get {
            ($t:ty) => {
                if let Ok(v) = self.try_get::<$t, _>(idx) {
                    return json!(v);
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

        Value::Null
    }
}

pub async fn rows_to_json_vec<R>(
    mut stream: impl Stream<Item = Result<R, SqlxError>> + Unpin,
) -> Result<Vec<Value>, SqlxError>
where
    R: JsonRow,
    usize: sqlx::ColumnIndex<R>,
{
    let mut results = Vec::new();

    while let Some(row) = stream.next().await {
        let row = row?;

        let mut map = Map::new();

        for i in 0..row.col_count() {
            let key = row.col_name(i).to_string();
            let val = row.get_json_value(i);
            map.insert(key, val);
        }

        results.push(Value::Object(map));
    }

    Ok(results)
}
