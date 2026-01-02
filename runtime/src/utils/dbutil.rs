use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use futures::{Stream, StreamExt};
use serde_json::{Map, Value, json};
use sqlx::{Column, Error as SqlxError, Row, postgres::PgRow};
use uuid::Uuid;

pub trait JsonRow {
    fn col_count(&self) -> usize;
    fn col_name(&self, idx: usize) -> &str;
    fn get_json_value(&self, idx: usize) -> Value;
}

impl JsonRow for PgRow {
    fn col_count(&self) -> usize {
        self.columns().len()
    }

    fn col_name(&self, idx: usize) -> &str {
        self.column(idx).name()
    }

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
        try_get!(Option<i64>);
        try_get!(Option<f64>);
        try_get!(Option<String>);
        try_get!(Option<DateTime<FixedOffset>>);
        try_get!(Option<serde_json::Value>);
        Value::Null
    }
}

pub trait StreamJsonExt {
    fn json(self, i: i32) -> impl std::future::Future<Output = Result<Vec<Value>, SqlxError>>;
}

impl<S> StreamJsonExt for S
where
    S: Stream<Item = Result<PgRow, SqlxError>> + Unpin,
{
    fn json(
        mut self,
        safe_limit: i32,
    ) -> impl std::future::Future<Output = Result<Vec<Value>, SqlxError>> {
        async move {
            let mut results = Vec::new();
            let mut i = 0;
            while let Some(row) = self.next().await {
                if (i >= safe_limit) {
                    break;
                }
                i = i + 1;
                let row = row?;
                let mut map = Map::new();

                for i in 0..row.col_count() {
                    map.insert(row.col_name(i).to_string(), row.get_json_value(i));
                }

                results.push(Value::Object(map));
            }

            Ok(results)
        }
    }
}
