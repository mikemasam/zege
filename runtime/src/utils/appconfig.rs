use std::{sync::OnceLock, time::Instant};

use anyhow::{Ok, Result};
use chrono::Local;
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

use crate::utils::logging::{AppLogger, LogLevel};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub verbose: Option<String>,
    pub port: Option<u16>,
    pub database: Database,
    pub redis: Option<RedisDatabase>,
    pub feature: Option<FeatureConfig>,
    pub auth: Option<AuthConfig>,
}

#[derive(Debug)]
pub struct AppState {
    pub startUp: Instant,
    pub config: AppConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: Option<String>,
    pub jwt_minutes: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub jwt_secret: Option<String>,
    #[serde(default = "default_no")]
    pub signup: Option<String>,
    pub login: Option<String>,
    pub create_bucket: Option<String>,
    pub create_organization: Option<String>,
    pub landing: Option<String>,
}

fn default_no() -> Option<String> {
    Some("no".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisDatabase {
    pub servers: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub driver: Option<String>,
    pub name: String,
    pub host: String,
    pub username: String,
    pub password: String,
}

impl AppConfig {
    pub fn init_config() -> Result<()> {
        let cfg: AppConfig = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config"))
            .add_source(Environment::with_prefix("APP"))
            .build()?
            .try_deserialize()
            .unwrap();
        APP_STATE
            .set(AppState {
                config: cfg,
                startUp: Instant::now(),
            })
            .expect("config already initialized");
        Ok(())
    }
    pub fn printlog() {
        let config = AppConfig::config();
        AppLogger::debug(serde_json::to_string_pretty(config).unwrap());
    }

    pub fn state() -> &'static AppState {
        APP_STATE.get().expect("app state not initialized")
    }
    pub fn config() -> &'static AppConfig {
        APP_STATE
            .get()
            .map(|s| &s.config)
            .expect("app state not initialized")
    }
    pub fn feature_enabled(&self, feature: AppFeature) -> bool {
        let f = match &self.feature {
            Some(f) => f,
            None => return false,
        };

        let raw = match feature {
            AppFeature::Signup => f.signup.as_deref(),
            AppFeature::Login => f.login.as_deref(),
            AppFeature::Landing => f.landing.as_deref(),
            AppFeature::CreateBucket => f.create_bucket.as_deref(),
            AppFeature::CreateOrganization => f.create_organization.as_deref(),
        };

        raw.map(|v| {
            let v = v.to_lowercase();
            v != "false" && v != "no"
        })
        .unwrap_or(true)
    }
}

static APP_STATE: OnceLock<AppState> = OnceLock::new();

#[macro_export]
macro_rules! appconfig {
    () => {
        crate::utils::appconfig::AppConfig::config()
    };
}

#[derive(Debug, Copy, Clone)]
pub enum AppFeature {
    Signup,
    Login,
    Landing,
    CreateBucket,
    CreateOrganization,
}
