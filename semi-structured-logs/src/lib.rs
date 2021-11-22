// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::fmt;

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

impl fmt::Display for LogLevel {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let tmp = match self {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
        LogLevel::Debug => "DEBUG",
    };
    write!(formatter, "{}", tmp)
  }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", level, message)
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
