mod color;
mod error;
mod format;
mod r#impl;
mod level;
mod macros;
mod target;
mod util;

use std::sync::OnceLock;

// Exports
pub use color::{Color, Colorize};
pub use error::Error;
pub use format::{DefaultFormatter, Formatter};
pub use level::LogLevel;
pub use target::{Console, File, Target};

#[cfg(feature = "blocking")]
pub use r#impl::blocking::DefaultLogger;

#[cfg(not(feature = "blocking"))]
pub use r#impl::r#async::{DefaultLogger, shutdown};

#[derive(Debug, Clone)]
pub struct Record {
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub module_path: Option<&'static str>,
    pub file: Option<&'static str>,
    pub line: Option<u32>,
}

pub trait Logger: Send + Sync {
    fn enabled(&self, level: LogLevel) -> bool;
    fn log(&self, record: &Record);

    #[cfg(not(feature = "blocking"))]
    fn abort(&self);
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
    let logger = DefaultLogger::new(config);

    set_logger(logger).expect("Failed to initalize logger");
}

pub fn init_default() {
    let logger = DefaultLogger::default();
    set_logger(logger).expect("Failed to initalize logger");
}

pub fn init_with_config(config: Config) {
    let logger = DefaultLogger::new(config);
    set_logger(logger).expect("Failed to initalize logger");
}
