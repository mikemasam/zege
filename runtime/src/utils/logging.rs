use chrono::Local;

use crate::{appconfig, utils::appconfig::AppConfig};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LogLevel {
    Error,
    Log,
    Debug,
}

impl LogLevel {
    pub fn from_config() -> Self {
        match appconfig!().verbose.as_deref() {
            Some("debug") | Some("all") => LogLevel::Debug,
            Some("log") | Some("info") => LogLevel::Log,
            _ => LogLevel::Error,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            LogLevel::Error => "error",
            LogLevel::Log => "log",
            LogLevel::Debug => "debug",
        }
    }
}

pub struct AppLogger {}

impl AppLogger {
    #[inline]
    fn enabled(level: LogLevel) -> bool {
        level <= LogLevel::from_config()
    }
    fn prefix(level: LogLevel) -> String {
        let uptime = AppConfig::state().startUp.elapsed();
        let total = uptime.as_secs();

        let days = total / 86_400;
        let hours = (total % 86_400) / 3_600;
        let minutes = (total % 3_600) / 60;
        let seconds = total % 60;

        let up = if days > 0 {
            format!(
                "{d}d{h:02}h{m:02}m{s:02}s",
                d = days,
                h = hours,
                m = minutes,
                s = seconds
            )
        } else if hours > 0 {
            format!("{h}h{m:02}m{s:02}s", h = hours, m = minutes, s = seconds)
        } else {
            format!("{m}m{s:02}s", m = minutes, s = seconds)
        };

        format!(
            "{}  up={}  {:<5}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            up,
            level.as_str().to_uppercase()
        )
    }

    pub fn write(level: LogLevel, msg: impl std::fmt::Display) {
        if !Self::enabled(level) {
            return;
        }

        println!("{} {}", Self::prefix(level), msg);
    }

    pub fn log(msg: impl std::fmt::Display) {
        AppLogger::write(LogLevel::Log, msg);
    }

    pub fn debug(msg: impl std::fmt::Display) {
        AppLogger::write(LogLevel::Debug, msg);
    }

    pub fn error(msg: impl std::fmt::Display) {
        AppLogger::write(LogLevel::Error, msg);
    }
}
