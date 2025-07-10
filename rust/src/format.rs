/// Formatting utilities for log messages.
use crate::Record;

/// Defines a log message formatter.
///
/// Formatters are responsible for converting a log record into a formatted
/// string that can be written to output targets.
pub trait Formatter: Send + Sync {
    /// Formats a log record into a string.
    ///
    /// # Arguments
    ///
    /// * `record` - The log record to format
    ///
    /// # Returns
    ///
    /// A formatted string representation of the log record
    fn format(&self, record: &Record) -> String;
}

/// Default log message formatter.
///
/// Creates log messages in the format: `[LEVEL] message`
/// with appropriate color coding for the log level.
pub struct DefaultFormatter;

impl Formatter for DefaultFormatter {
    /// Formats a log record using the default format.
    ///
    /// The default format is `[LEVEL] message` where LEVEL is color-coded
    /// according to severity.
    ///
    /// # Arguments
    ///
    /// * `record` - The log record to format
    ///
    /// # Returns
    ///
    /// A formatted string representation of the log record
    fn format(&self, record: &Record) -> String {
        format!("[{}] {}", record.level.default_coloring(), record.message)
    }
}
