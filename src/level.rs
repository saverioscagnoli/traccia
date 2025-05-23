use std::str::FromStr;

/// Log level definitions and utilities.
use crate::{Color, Colorize};

/// Logging severity levels in ascending order of importance.
///
/// The levels follow the common convention:
/// - `Trace`: Very detailed information, typically only needed when debugging specific issues
/// - `Debug`: Detailed information useful for debugging
/// - `Info`: General information about application progress
/// - `Warn`: Potentially harmful situations that might need attention
/// - `Error`: Error events that might still allow the application to continue running
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    /// Very detailed information for debugging specific issues
    Trace,
    /// Detailed information useful for debugging
    Debug,
    /// General information about application progress
    Info,
    /// Potentially harmful situations
    Warn,
    /// Error events, Could still allow the application to continue running
    Error,
    /// Fatal error events that lead to application termination
    Fatal,
}

impl std::fmt::Display for LogLevel {
    /// Formats the log level as a string.
    ///
    /// # Returns
    ///
    /// A capitalized string representation of the log level.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Fatal => write!(f, "FATAL"),
        }
    }
}

impl LogLevel {
    /// Returns the level string with appropriate color formatting.
    ///
    /// Each log level has an associated color:
    /// - Trace: Cyan
    /// - Debug: Blue
    /// - Info: Green
    /// - Warn: Yellow
    /// - Error: Red
    ///
    /// # Returns
    ///
    /// The formatted string with ANSI color codes applied
    pub fn default_coloring(&self) -> String {
        match self {
            LogLevel::Trace => format!("{}", self).color(Color::Cyan),
            LogLevel::Debug => format!("{}", self).color(Color::Blue),
            LogLevel::Info => format!("{}", self).color(Color::Green),
            LogLevel::Warn => format!("{}", self).color(Color::Yellow),
            LogLevel::Error => format!("{}", self).color(Color::Red),
            LogLevel::Fatal => format!("{}", self).color(Color::BrightRed),
        }
    }
}

/// The default log level is info
impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

/// Parse the log level from &str.
///
/// Useful for things like clap to parse the log level via
/// command-line arguments.
impl FromStr for LogLevel {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            "fatal" => Ok(LogLevel::Fatal),
            _ => Err(crate::Error::ParseLogLevel),
        }
    }
}

/// Tryfrom u8 parsing implementation
impl TryFrom<u8> for LogLevel {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, crate::Error> {
        match value {
            0 => Ok(LogLevel::Trace),
            1 => Ok(LogLevel::Debug),
            2 => Ok(LogLevel::Info),
            3 => Ok(LogLevel::Warn),
            4 => Ok(LogLevel::Error),
            5 => Ok(LogLevel::Fatal),
            _ => Err(crate::Error::ParseLogLevel),
        }
    }
}

/// TryFrom log level implementation for u8
impl TryFrom<LogLevel> for u8 {
    type Error = crate::Error;

    fn try_from(value: LogLevel) -> Result<Self, Self::Error> {
        match value {
            LogLevel::Trace => Ok(0),
            LogLevel::Debug => Ok(1),
            LogLevel::Info => Ok(2),
            LogLevel::Warn => Ok(3),
            LogLevel::Error => Ok(4),
            LogLevel::Fatal => Ok(5),
        }
    }
}

/// Implementation of the clap's ValueEnum trait for LogLevel when the "clap" feature is enabled.
///
/// This allows using LogLevel directly with clap's derive API and provides several ways to
/// specify log levels on the command line:
///
/// # Command-line examples:
/// ```
/// # Using lowercase (default format)
/// myapp --log-level debug
///
/// # Using uppercase
/// myapp --log-level DEBUG
///
/// # Using title case
/// myapp --log-level Debug
/// ```
///
/// This implementation provides the list of all possible log level variants and defines
/// how they can be specified from command-line arguments, supporting case-insensitive matching.
#[cfg(feature = "clap")]
impl clap::ValueEnum for LogLevel {
    /// Returns an array slice containing all variants of LogLevel.
    ///
    /// This is used by clap to validate and parse command-line arguments.
    fn value_variants<'a>() -> &'a [Self] {
        &[
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
            LogLevel::Fatal,
        ]
    }

    /// Converts a LogLevel variant to its string representation with aliases.
    ///
    /// For each log level, this provides:
    /// - A lowercase default (e.g., "debug")
    /// - An uppercase alias (e.g., "DEBUG")
    /// - A title case alias (e.g., "Debug")
    ///
    /// This allows users to specify the log level in any case they prefer.
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            LogLevel::Trace => clap::builder::PossibleValue::new("trace")
                .alias("TRACE")
                .alias("Trace"),
            LogLevel::Debug => clap::builder::PossibleValue::new("debug")
                .alias("DEBUG")
                .alias("Debug"),
            LogLevel::Info => clap::builder::PossibleValue::new("info")
                .alias("INFO")
                .alias("Info"),
            LogLevel::Warn => clap::builder::PossibleValue::new("warn")
                .alias("WARN")
                .alias("Warn"),
            LogLevel::Error => clap::builder::PossibleValue::new("error")
                .alias("ERROR")
                .alias("Error"),
            LogLevel::Fatal => clap::builder::PossibleValue::new("fatal")
                .alias("FATAL")
                .alias("Fatal"),
        })
    }
}
