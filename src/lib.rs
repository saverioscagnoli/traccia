//! A lightweight, flexible logging framework for Rust applications.
//!
//! This crate provides a configurable logging system that supports multiple output targets,
//! customizable formatting, and different log levels. It can be used in both synchronous (blocking)
//! and asynchronous contexts.
//!
//! # Features
//!
//! * Multiple log levels (Debug, Info, Warning, Error)
//! * Multiple output targets (Console, File)
//! * Customizable log formatting
//! * Async and blocking implementations
//!
//! # Example
//!
//! ```rust,ignore
//! use logger::{init, LogLevel};
//!
//! // Initialize with Info level
//! init(LogLevel::Info);
//!
//! // Log messages
//! info!("Application started");
//! debug!("This won't be displayed with Info level");
//! error!("Something went wrong: {}", error);
//! ```
mod error;
mod format;
mod r#impl;
mod level;
mod macros;
mod strings;
mod target;
mod util;

#[cfg(not(feature = "blocking"))]
mod shutdown;

use std::{sync::OnceLock, thread::ThreadId};

// Exports
pub use error::Error;
pub use format::{DefaultFormatter, Formatter};
pub use level::LogLevel;
pub use strings::{Color, Colorize, Style};
pub use target::{Console, File, FileMode, Target};

#[cfg(feature = "blocking")]
pub use r#impl::blocking::DefaultLogger;

#[cfg(not(feature = "blocking"))]
pub use r#impl::r#async::DefaultLogger;

/// Represents a single log record with all relevant metadata.
///
/// A `Record` contains the log level, target component, message content, and
/// source location information (module path, file, line).
#[derive(Debug, Clone)]
pub struct Record {
    /// The severity level of the log message.
    pub level: LogLevel,

    /// The thread ID where the log was generated.
    pub thread_id: ThreadId,

    /// The target component or category for the log message.
    pub target: String,

    /// The actual log message content.
    pub message: String,

    /// Optional module path where the log was generated.
    pub module_path: Option<&'static str>,

    /// Optional source code file where the log was generated.
    pub file: Option<&'static str>,

    /// Optional line number in the source code where the log was generated.
    pub line: Option<u32>,
}

/// Core trait that defines the logging behavior.
///
/// Implementors of this trait handle the actual processing and writing of log records.
/// Custom loggers can be created by implementing this trait.
pub trait Logger: Send + Sync {
    /// Determines if a message with the given log level should be processed.
    ///
    /// # Arguments
    ///
    /// * `level` - The log level to check
    ///
    /// # Returns
    ///
    /// `true` if messages at this level should be logged, `false` otherwise
    fn enabled(&self, level: LogLevel) -> bool;

    /// Process and output a log record.
    ///
    /// # Arguments
    ///
    /// * `record` - The log record to process
    fn log(&self, record: &Record);

    /// Abort any ongoing logging operations and cleanup resources.
    ///
    /// This method is only available when not using the "blocking" feature.
    #[cfg(not(feature = "blocking"))]
    fn abort(&self);
}

/// Configuration for initializing a logger.
///
/// This struct allows customizing the logger's behavior by specifying
/// the minimum log level, output targets, and formatting options.
pub struct Config {
    /// The minimum log level that will be processed.
    pub level: LogLevel,

    /// List of targets where log messages will be sent.
    pub targets: Vec<Box<dyn Target>>,

    /// Optional formatter for customizing log message appearance.
    pub format: Option<Box<dyn Formatter>>,
}

impl Config {
    /// Creates a default configuration with the specified log level.
    ///
    /// The configuration uses console output with default formatting.
    ///
    /// # Arguments
    ///
    /// * `level` - The minimum log level to use
    ///
    /// # Returns
    ///
    /// A new `Config` instance with default settings except for the specified level
    pub fn default_with_level(level: LogLevel) -> Self {
        Config {
            level,
            targets: vec![Box::new(target::Console)],
            format: Some(Box::new(format::DefaultFormatter)),
        }
    }
}

impl Default for Config {
    /// Creates a default configuration with `Info` log level.
    ///
    /// The configuration uses console output with default formatting.
    ///
    /// # Returns
    ///
    /// A new default `Config` instance
    fn default() -> Self {
        Config {
            level: LogLevel::Info,
            targets: vec![Box::new(target::Console)],
            format: Some(Box::new(format::DefaultFormatter)),
        }
    }
}

/// Global logger instance storage.
///
/// This static variable holds the singleton logger instance once initialized.
static LOGGER: OnceLock<Box<dyn Logger>> = OnceLock::new();

/// Sets the global logger instance.
///
/// This function can only be called once. Subsequent calls will return an error.
///
/// # Arguments
///
/// * `logger` - The logger implementation to use
///
/// # Returns
///
/// `Ok(())` if successful, or `Error::AlreadyInitialized` if a logger is already set
fn set_logger<L: Logger + 'static>(logger: L) -> Result<(), Error> {
    match LOGGER.set(Box::new(logger)) {
        Ok(_) => {
            #[cfg(not(feature = "blocking"))]
            shutdown::add_hook(|| {
                if let Some(logger) = LOGGER.get() {
                    logger.abort();
                }
            });

            Ok(())
        }
        Err(_) => Err(Error::AlreadyInitialized),
    }
}

/// Retrieves the global logger instance.
///
/// # Returns
///
/// A reference to the logger if initialized, or `Error::NotInitialized` if not
pub fn logger() -> Result<&'static Box<dyn Logger>, Error> {
    LOGGER.get().ok_or(Error::NotInitialized)
}

/// Initializes the global logger with the specified minimum log level.
///
/// This function creates a logger with default configuration except for the
/// specified log level. It is a convenience wrapper around `init_with_config`.
///
/// # Arguments
///
/// * `level` - The minimum log level to use
///
/// # Panics
///
/// Panics if a logger is already initialized
pub fn init(level: LogLevel) {
    let config = Config::default_with_level(level);
    let logger = DefaultLogger::new(config);

    set_logger(logger).expect("Failed to initalize logger");
}

/// Initializes the global logger with default settings.
///
/// This function creates a logger with the default configuration (Info level, console output).
///
/// # Panics
///
/// Panics if a logger is already initialized
pub fn init_default() {
    let logger = DefaultLogger::default();
    set_logger(logger).expect("Failed to initalize logger");
}

/// Initializes the global logger with a custom configuration.
///
/// This function allows full customization of the logger behavior.
///
/// # Arguments
///
/// * `config` - The configuration to use
///
/// # Panics
///
/// Panics if a logger is already initialized
pub fn init_with_config(config: Config) {
    let logger = DefaultLogger::new(config);
    set_logger(logger).expect("Failed to initalize logger");
}
