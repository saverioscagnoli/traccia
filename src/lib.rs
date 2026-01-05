pub mod target;

mod defaults;
mod formatter;
mod macros;
mod style;

use crate::{
    defaults::{DefaultFormatter, DefaultLogger},
    formatter::Formatter,
    target::{Console, Target},
};
use std::{fmt, sync::OnceLock};

// Re-exports
pub use style::*;

#[derive(Default)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    Fatal,
    Custom(&'static str),
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trace => write!(f, "trace"),
            Self::Debug => write!(f, "debug"),
            Self::Info => write!(f, "info"),
            Self::Warn => write!(f, "warn"),
            Self::Error => write!(f, "error"),
            Self::Fatal => write!(f, "fatal"),
            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: u64,
    pub target: String,
}

pub trait Logger: Send + Sync {
    fn enabled(&self, level: LogLevel) -> bool;

    fn log(&self, record: &Record);

    fn flush(&mut self);
}

static LOGGER: OnceLock<Box<dyn Logger>> = OnceLock::new();

pub fn logger() -> &'static Box<dyn Logger> {
    LOGGER.get().expect("Logger not initialized")
}

pub fn set_logger(logger: Box<dyn Logger>) {
    LOGGER
        .set(logger)
        .map_err(|_| String::from("Logger already initialized"))
        .unwrap();
}

pub struct Config {
    pub level: LogLevel,
    pub targets: Vec<Box<dyn Target>>,
    pub formatter: Box<dyn Formatter>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            level: LogLevel::default(),
            targets: vec![Box::new(Console::new())],
            formatter: Box::new(DefaultFormatter),
        }
    }
}

impl Config {
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    pub fn with_targets(mut self, targets: Vec<Box<dyn Target>>) -> Self {
        self.targets = targets;
        self
    }

    pub fn with_formatter(mut self, formatter: Box<dyn Formatter>) -> Self {
        self.formatter = formatter;
        self
    }
}

// === INIT ===

pub fn init(config: Config) {
    set_logger(Box::new(DefaultLogger::new(config)));
}

pub fn init_with_level(level: LogLevel) {
    let config = Config::default().with_level(level);
    init(config);
}

pub fn init_default() {
    set_logger(Box::new(DefaultLogger::default()));
}
