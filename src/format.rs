use crate::{
    LogLevel, Record,
    color::{Color, Colorize},
};

pub trait Formatter: Send + Sync {
    fn format(&self, record: &Record) -> String;
}

pub struct DefaultFormatter;

impl Formatter for DefaultFormatter {
    fn format(&self, record: &Record) -> String {
        let level = match record.level {
            LogLevel::Trace => "TRACE".color(Color::Cyan),
            LogLevel::Debug => "DEBUG".color(Color::Blue),
            LogLevel::Info => "INFO".color(Color::Green),
            LogLevel::Warn => "WARN".color(Color::Yellow),
            LogLevel::Error => "ERROR".color(Color::Red),
        };

        format!("[{}] {}", level, record.message)
    }
}
