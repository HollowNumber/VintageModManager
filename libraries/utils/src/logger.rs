use chrono::Local;
use colored::*;
use std::fmt;

#[derive(Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        };

        let colored_level = match self {
            LogLevel::Debug => level_str.blue(),
            LogLevel::Info => level_str.green(),
            LogLevel::Warn => level_str.yellow(),
            LogLevel::Error => level_str.red(),
        };

        write!(f, "{}", colored_level)
    }
}

pub struct Logger {
    pub logger_name: String,
    pub default_log_level: LogLevel,
}

impl Logger {
    pub fn new(logger_name: String, default_log_level: LogLevel) -> Logger {
        Self {
            logger_name,
            default_log_level,
        }
    }

    // how do we make level optional?

    pub fn log(&self, level: LogLevel, message: &str) {
        // European time format
        let current_time = Local::now().format("%Y-%d-%m %H:%M:%S").to_string();

        println!(
            "{} [{}] {}: {}",
            current_time, level, self.logger_name, message
        );
    }

    pub fn log_default(&self, message: &str) {
        self.log(self.default_log_level.clone(), message);
    }
}
