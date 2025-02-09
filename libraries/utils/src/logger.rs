use chrono::Local;
use std::cell::RefCell;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::rc::Rc;

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

        write!(f, "{}", level_str)
    }
}

/// Struct representing a logger.
pub struct Logger {
    /// The name of the logger.
    pub logger_name: String,
    /// The default log level for the logger.
    pub default_log_level: LogLevel,
    /// The file handle for logging to a file.
    file: Rc<RefCell<std::fs::File>>,
}

impl Logger {
    /// Creates a new `Logger`.
    ///
    /// # Arguments
    ///
    /// * `logger_name` - A `String` representing the name of the logger.
    /// * `default_log_level` - The default `LogLevel` for the logger.
    /// * `file_path` - The path to the log file.
    ///
    /// # Returns
    ///
    /// A new `Logger` instance.
    pub fn new(logger_name: String, default_log_level: LogLevel, file_path: &str) -> Logger {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Unable to open log file");

        Self {
            logger_name,
            default_log_level,
            file: Rc::new(RefCell::new(file)),
        }
    }

    /// Logs a message with the specified log level.
    ///
    /// # Arguments
    ///
    /// * `level` - The `LogLevel` to log the message with.
    /// * `message` - A `&str` representing the message to log.
    pub fn log(&self, level: LogLevel, message: &str) {
        let current_time = Local::now().format("%Y-%d-%m %H:%M:%S").to_string();

        let log_message = format!(
            "{} [{}] {}: {}\n",
            current_time, level, self.logger_name, message
        );

        // Print to console
        println!("{}", log_message);

        // Write to file without color codes
        let file_log_message = format!(
            "{} [{}] {}: {}\n",
            current_time, level, self.logger_name, message
        );

        self.file
            .borrow_mut()
            .write_all(file_log_message.as_bytes())
            .expect("Unable to write to log file");
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
