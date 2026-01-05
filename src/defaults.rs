//! This module contains general-purpose default implementations
//! for different components of the library.

use crate::{Color, Colorize, Config, LogLevel, Logger, Record, Style, formatter::Formatter};

pub struct DefaultLogger {
    config: Config,
}

unsafe impl Send for DefaultLogger {}
unsafe impl Sync for DefaultLogger {}

impl Logger for DefaultLogger {
    fn enabled(&self, level: crate::LogLevel) -> bool {
        self.config.level <= level
    }

    fn log(&self, record: &crate::Record) {
        if !self.enabled(record.level) {
            return;
        }

        for target in &self.config.targets {
            let binding = target.formatter();
            let formatter = binding.as_deref().unwrap_or(&*self.config.formatter);

            let message = formatter.format(record);

            if let Err(e) = target.write(record.level, message) {
                eprintln!("Failed to write log message: {}", e);
            }
        }
    }

    fn flush(&mut self) {}
}

impl Default for DefaultLogger {
    fn default() -> Self {
        Self {
            config: Config::default(),
        }
    }
}

impl DefaultLogger {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

pub struct DefaultFormatter;

impl Formatter for DefaultFormatter {
    fn format(&self, record: &Record) -> String {
        format!("[{}]: {}", record.level, record.message)
    }
}

pub struct ConsoleFormatter;

impl Formatter for ConsoleFormatter {
    fn format(&self, record: &Record) -> String {
        let level = match record.level {
            LogLevel::Trace => record.level.to_string().color(Color::Cyan),
            LogLevel::Debug => record.level.to_string().color(Color::Blue),
            LogLevel::Info => record.level.to_string().color(Color::Green),
            LogLevel::Warn => record.level.to_string().color(Color::Yellow),
            LogLevel::Error => record.level.to_string().color(Color::Red),
            LogLevel::Fatal => record.level.to_string().color(Color::Red).bold(),
            LogLevel::Custom(name) => name.to_string(),
        };

        format!("{} [{}]: {}", record.timestamp, level, record.message)
    }
}
