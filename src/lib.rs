mod color;
mod error;
mod format;
mod macros;
mod target;
mod util;

use std::{fmt::Display, sync::OnceLock};

// Exports
pub use color::{Color, Colorize};
pub use error::Error;
pub use format::{DefaultFormatter, Formatter};
pub use target::{Console, File, Target};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

impl LogLevel {
    pub fn default_coloring(&self) -> String {
        match self {
            LogLevel::Trace => format!("{}", self).color(Color::Cyan),
            LogLevel::Debug => format!("{}", self).color(Color::Blue),
            LogLevel::Info => format!("{}", self).color(Color::Green),
            LogLevel::Warn => format!("{}", self).color(Color::Yellow),
            LogLevel::Error => format!("{}", self).color(Color::Red),
        }
    }
}

pub struct Record {
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub module_path: Option<&'static str>,
    pub file: Option<&'static str>,
    pub line: Option<u32>,
}

pub trait Logger: Send + Sync {
    fn log(&self, record: &Record);
    fn enabled(&self, level: LogLevel) -> bool;
}

pub struct Config {
    pub level: LogLevel,
    pub targets: Vec<Box<dyn Target>>,
    pub format: Option<Box<dyn Formatter>>,
}

impl Config {
    pub fn default_with_level(level: LogLevel) -> Self {
        Config {
            level,
            targets: vec![Box::new(target::Console)],
            format: Some(Box::new(format::DefaultFormatter)),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            level: LogLevel::Info,
            targets: vec![Box::new(target::Console)],
            format: Some(Box::new(format::DefaultFormatter)),
        }
    }
}

pub struct AsyncLogger {
    config: Config,
}

impl AsyncLogger {
    pub fn new(config: Config) -> Self {
        AsyncLogger { config }
    }
}

impl Default for AsyncLogger {
    fn default() -> Self {
        AsyncLogger::new(Config::default())
    }
}

impl Logger for AsyncLogger {
    fn enabled(&self, level: LogLevel) -> bool {
        self.config.level <= level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.level) {
            return;
        }

        let formatted = match &self.config.format {
            Some(formatter) => formatter.format(record),
            None => format::DefaultFormatter.format(record),
        };

        for target in &self.config.targets {
            if let Err(e) = target.write(&formatted) {
                eprintln!("Failed to write to target: {}", e);
            }
        }
    }
}

static LOGGER: OnceLock<Box<dyn Logger>> = OnceLock::new();

fn set_logger<L: Logger + 'static>(logger: L) -> Result<(), Error> {
    match LOGGER.set(Box::new(logger)) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::AlreadyInitialized),
    }
}

pub fn logger() -> Result<&'static Box<dyn Logger>, Error> {
    LOGGER.get().ok_or(Error::NotInitialized)
}

pub fn init(level: LogLevel) {
    let config = Config::default_with_level(level);
    let logger = AsyncLogger::new(config);

    set_logger(logger).expect("Failed to initalize logger");
}

pub fn init_with_config(config: Config) {
    let logger = AsyncLogger::new(config);

    set_logger(logger).expect("Failed to initalize logger");
}
