use std::{sync::OnceLock, time::Instant};

use anyhow::{Ok, Result};
use chrono::Local;
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

use crate::utils::appconfig;

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
    pub signup: Option<String>,
    pub login: Option<String>,
    pub create_bucket: Option<String>,
    pub create_organization: Option<String>,
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
        applogger::debug(serde_json::to_string_pretty(config).unwrap());
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
}

static APP_STATE: OnceLock<AppState> = OnceLock::new();

#[macro_export]
macro_rules! appconfig {
    () => {
        crate::utils::appconfig::AppConfig::config()
    };
}

pub struct applogger {}

impl applogger {
    pub fn mark(label: &str) -> String {
        let delta = AppConfig::state().startUp.elapsed();
        let d_secs = delta.as_secs();
        let days = d_secs / 86_400;
        let hours = (d_secs % 86_400) / 3_600;
        let minutes = (d_secs % 3_600) / 60;
        let secs = d_secs % 60;
        return format!(
            "{} - {}<{}d{}h{}m{}s> ",
            Local::now().format("%Y-%d-%mT%H:%M:%S"),
            label,
            days,
            hours,
            minutes,
            secs
        );
    }
    pub fn info(str: String) {
        let enabled = matches!(
            appconfig!().verbose.as_deref(),
            Some("all") | Some("info") | Some("log") | Some("debug")
        );
        if (!enabled) {
            return;
        }
        println!("{}{}", applogger::mark("info"), str);
    }
    pub fn log(str: String) {
        let enabled = matches!(appconfig!().verbose.as_deref(), Some("all") | Some("log"));
        if (!enabled) {
            return;
        }
        println!("{}{}", applogger::mark("log"), str);
    }
    pub fn debug(str: String) {
        let enabled = matches!(appconfig!().verbose.as_deref(), Some("all") | Some("debug"));
        if (!enabled) {
            return;
        }
        println!("{}{}", applogger::mark("debug"), str);
    }
    pub fn error(str: String) {
        println!("{}{}", applogger::mark("error"), str);
    }
}
