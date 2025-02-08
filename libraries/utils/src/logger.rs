use chrono::Local;
use colored::*;
use std::fmt;

/// Enum representing different log levels.
#[derive(Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    /// Formats the `LogLevel` for display.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to the formatter.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating the result of the formatting operation.
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

/// Struct representing a logger.
pub struct Logger {
    /// The name of the logger.
    pub logger_name: String,
    /// The default log level for the logger.
    pub default_log_level: LogLevel,
}

impl Logger {
    /// Creates a new `Logger`.
    ///
    /// # Arguments
    ///
    /// * `logger_name` - A `String` representing the name of the logger.
    /// * `default_log_level` - The default `LogLevel` for the logger.
    ///
    /// # Returns
    ///
    /// A new `Logger` instance.
    pub fn new(logger_name: String, default_log_level: LogLevel) -> Logger {
        Self {
            logger_name,
            default_log_level,
        }
    }

    /// Logs a message with the specified log level.
    ///
    /// # Arguments
    ///
    /// * `level` - The `LogLevel` to log the message with.
    /// * `message` - A `&str` representing the message to log.
    pub fn log(&self, level: LogLevel, message: &str) {
        // European time format
        let current_time = Local::now().format("%Y-%d-%m %H:%M:%S").to_string();

        println!(
            "{} [{}] {}: {}",
            current_time, level, self.logger_name, message
        );
    }

    /// Logs a message with the default log level.
    ///
    /// # Arguments
    ///
    /// * `message` - A `&str` representing the message to log.
    pub fn log_default(&self, message: &str) {
        self.log(self.default_log_level.clone(), message);
    }
}
