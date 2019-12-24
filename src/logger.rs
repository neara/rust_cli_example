use chrono::prelude::Utc;
use serde_json;
use std::collections::HashMap;
use std::fmt;

pub const INFO: &str = "INFO";
pub const DEBUG: &str = "DEBUG";
pub const ERROR: &str = "ERROR";

pub type LogExtraParams<'a> = HashMap<&'a str, &'a str>;

#[derive(Copy, Clone)]
enum LogLevel {
    DEBUG,
    INFO,
    ERROR,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::DEBUG => DEBUG,
            Self::INFO => INFO,
            Self::ERROR => ERROR,
        };

        write!(f, "{}", name)
    }
}

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn get_logger(level: Option<&str>) -> Logger {
        // return initialized Logger instance
        let lvl = match level {
            Some(log_level) => match log_level {
                DEBUG => LogLevel::DEBUG,
                INFO => LogLevel::INFO,
                ERROR => LogLevel::ERROR,
                _ => LogLevel::INFO,
            },
            None => LogLevel::INFO,
        };

        Logger { level: lvl }
    }

    /// Format and output message if debug is turned on
    pub fn debug(&self, message: &str, extra_params: Option<&LogExtraParams>) {
        if let LogLevel::DEBUG = self.level {
            println!("{}", self.make_message(message, extra_params))
        }
    }

    /// Format and output message if level is INFO
    pub fn info(&self, message: &str, extra_params: Option<&LogExtraParams>) {
        if let LogLevel::INFO = self.level {
            println!("{}", self.make_message(message, extra_params))
        }
    }

    /// Format and output error message
    pub fn error(&self, message: &str, extra_params: Option<&LogExtraParams>) {
        if let LogLevel::ERROR = self.level {
            eprintln!("{}", self.make_message(message, extra_params))
        }
    }

    fn make_message(&self, msg: &str, extra_params: Option<&LogExtraParams>) -> String {
        let extra = match extra_params {
            None => String::new(),
            Some(params) => serde_json::to_string(params).unwrap_or_else(|err| {
                println!("Failed to serialize extra params: {}", err);
                String::new()
            }),
        };

        format!(
            "{date_time}     {level}     {message}  {extra}",
            date_time = Utc::now(),
            level = self.level,
            message = msg,
            extra = extra
        )
    }
}
