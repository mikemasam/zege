use std::collections::HashMap;

use anyhow::{Result, ensure};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{ctx::appcontext::DbStorage, lib::buckets::Bucket, utils::security::Security};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEventInput {
    pub bucket_key: Option<String>,
    pub event_organization_id: Option<i64>,
    pub event_bucket_id: Option<i64>,
    pub timestamp: DateTime<FixedOffset>,
    pub service: String,
    pub version: Option<String>,
    pub host: Option<String>,
    pub event_name: String,
    pub event_type: Option<String>,
    pub message: Option<String>,
    pub meta: Option<Meta>,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    pub jwt: Option<String>,
}

pub enum LogEventChannelMessage {
    Data(Box<LogEventInput>),
    Flush,
}

impl LogEventInput {
    pub async fn validate_limits(&self) -> Result<()> {
        ensure!(self.service.len() < 250, "service cannot exceed 250");
        ensure!(self.event_name.len() < 250, "event_name cannot exceed 250");
        ensure!(
            self.version.as_ref().map(|s| s.len()).unwrap_or(0) < 250,
            "version cannot exceed 250"
        );
        ensure!(
            self.host.as_ref().map(|s| s.len()).unwrap_or(0) < 250,
            "host cannot exceed 250"
        );
        ensure!(
            self.event_type.as_ref().map(|s| s.len()).unwrap_or(0) < 250,
            "event_type cannot exceed 250"
        );
        ensure!(
            self.message.as_ref().map(|s| s.len()).unwrap_or(0) < 5000,
            "message cannot exceed 5000"
        );
        Ok(())
    }

    pub async fn inject(&mut self, db: DbStorage) -> Result<()> {
        self.validate_limits().await;
        let bucket =
            Bucket::find_by_apikey(db.clone(), self.bucket_key.as_ref().unwrap().to_string()).await;

        ensure!(
            bucket.is_ok(),
            format!(
                "bucket api key not found {} for event {}",
                self.bucket_key.as_ref().unwrap(),
                self.event_name
            )
        );
        let s = bucket.unwrap();

        self.event_bucket_id = Some(s.id);
        self.event_organization_id = Some(s.organization_id);
        self.injectJwt(db).await;
        Ok(())
    }
    pub async fn injectJwt(&mut self, db: DbStorage) -> Result<()> {
        if let Some(jwt) = self
            .meta
            .as_ref()
            .and_then(|u| u.jwt.as_deref())
            .filter(|jwt| !jwt.is_empty())
            .and_then(|jwt| Security::unzip_jwt(jwt).ok())
        {
            if (self.data.is_none()) {
                self.data = Some(HashMap::new());
            }
            self.data
                .as_mut()
                .unwrap()
                .insert("meta_jwt".to_string(), jwt);
        }
        Ok(())
    }
}
