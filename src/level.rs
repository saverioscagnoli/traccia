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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Very detailed information for debugging specific issues
    Trace,
    /// Detailed information useful for debugging
    Debug,
    /// General information about application progress
    Info,
    /// Potentially harmful situations
    Warn,
    /// Error events
    Error,
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
        }
    }
}
