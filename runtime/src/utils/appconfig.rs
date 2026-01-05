use std::sync::OnceLock;

use anyhow::{Ok, Result};
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub verbose: Option<String>,
    pub port: Option<u16>,
    pub database: Database,
    pub redis: RedisDatabase,
    pub feature: Option<FeatureConfig>,
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
        CONFIG.set(cfg).expect("config already initialized");
        Ok(())
    }

    pub fn config() -> &'static AppConfig {
        CONFIG.get().expect("config not initialized")
    }
}

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[macro_export]
macro_rules! appconfig {
    () => {
        crate::utils::appconfig::AppConfig::config()
    };
}

pub struct applogger {}

impl applogger {
    pub fn log(str: String) {
        let enabled = matches!(appconfig!().verbose.as_deref(), Some("all") | Some("log"));
        if (!enabled) {
            return;
        }
        println!("LOG: > {}", str);
    }
    pub fn debug(str: String) {
        let enabled = matches!(
            appconfig!().verbose.as_deref(),
            Some("all") | Some("log") | Some("debug")
        );
        if (!enabled) {
            return;
        }
        println!("DEBUG: ? {}", str);
    }
    pub fn error(str: String) {
        println!("ERR: ! {}", str);
    }
}
