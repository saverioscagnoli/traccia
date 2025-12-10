/// Formatting utilities for log messages.
use crate::Record;

/// Position where span context should appear in log messages.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SpanPosition {
    #[default]
    /// Span context appears at the end of the message (default).
    /// Example: `[INFO] message [span: key=value]`
    End,

    /// Span context appears at the beginning, before the level.
    /// Example: `[span: key=value] [INFO] message`
    Start,

    /// Span context appears after the level, before the message.
    /// Example: `[INFO] [span: key=value] message`
    AfterLevel,

    /// Span context is not included in the output.
    None,
}

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

/// Default log message formatter with configurable span positioning.
///
/// Creates log messages in the format: `[LEVEL] message`
/// with appropriate color coding for the log level and optional span context.
///
/// # Examples
///
/// ```rust,ignore
/// use traccia::{Config, LogLevel, DefaultFormatter, SpanPosition, Console};
///
/// // Default formatter (span at end)
/// let formatter = DefaultFormatter::new();
///
/// // Span at start
/// let formatter = DefaultFormatter::with_span_at_start();
///
/// // Custom position
/// let formatter = DefaultFormatter::with_position(SpanPosition::AfterLevel);
///
/// let config = Config {
///     level: LogLevel::Info,
///     targets: vec![Box::new(Console::new())],
///     format: Some(Box::new(formatter)),
/// };
/// ```
pub struct DefaultFormatter {
    /// The position where span context should appear.
    pub position: SpanPosition,
}

impl DefaultFormatter {
    /// Creates a new formatter with the specified span position.
    ///
    /// # Arguments
    ///
    /// * `position` - Where to place span context in the log message
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use traccia::{DefaultFormatter, SpanPosition};
    ///
    /// let formatter = DefaultFormatter::with_position(SpanPosition::Start);
    /// ```
    pub fn with_position(position: SpanPosition) -> Self {
        Self { position }
    }

    /// Creates a new formatter with default settings (span at end).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use traccia::DefaultFormatter;
    ///
    /// let formatter = DefaultFormatter::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a formatter with span context at the start.
    ///
    /// Output format: `[span: key=value] [LEVEL] message`
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use traccia::DefaultFormatter;
    ///
    /// let formatter = DefaultFormatter::with_span_at_start();
    /// ```
    pub fn with_span_at_start() -> Self {
        Self::with_position(SpanPosition::Start)
    }

    /// Creates a formatter with span context after the level.
    ///
    /// Output format: `[LEVEL] [span: key=value] message`
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use traccia::DefaultFormatter;
    ///
    /// let formatter = DefaultFormatter::with_span_after_level();
    /// ```
    pub fn with_span_after_level() -> Self {
        Self::with_position(SpanPosition::AfterLevel)
    }

    /// Creates a formatter with span context at the end (default).
    ///
    /// Output format: `[LEVEL] message [span: key=value]`
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use traccia::DefaultFormatter;
    ///
    /// let formatter = DefaultFormatter::with_span_at_end();
    /// ```
    pub fn with_span_at_end() -> Self {
        Self::with_position(SpanPosition::End)
    }

    /// Creates a formatter without span context.
    ///
    /// Output format: `[LEVEL] message`
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use traccia::DefaultFormatter;
    ///
    /// let formatter = DefaultFormatter::without_span();
    /// ```
    pub fn without_span() -> Self {
        Self::with_position(SpanPosition::None)
    }
}

impl Default for DefaultFormatter {
    fn default() -> Self {
        Self {
            position: SpanPosition::End,
        }
    }
}

impl Formatter for DefaultFormatter {
    /// Formats a log record using the configured format.
    ///
    /// The format includes the log level (color-coded) and message,
    /// with span context positioned according to the formatter's configuration.
    ///
    /// # Arguments
    ///
    /// * `record` - The log record to format
    ///
    /// # Returns
    ///
    /// A formatted string representation of the log record
    fn format(&self, record: &Record) -> String {
        format_with_span_position(record, self.position)
    }
}

/// Formats span context into a string with default formatting.
///
/// Each span is formatted as `[span_name: fields]` and multiple spans are joined with spaces.
///
/// # Arguments
///
/// * `context` - Slice of (span_name, fields) tuples
///
/// # Returns
///
/// A formatted string of all span contexts, or an empty string if no context exists
///
/// # Examples
///
/// ```rust,ignore
/// use traccia::format_span_context;
///
/// let context = vec![
///     ("request".to_string(), "id=123".to_string()),
///     ("user".to_string(), "name=john".to_string()),
/// ];
/// let span_str = format_span_context(&context);
/// // Returns: "[request: id=123] [user: name=john]"
/// ```
pub fn format_span_context(context: &[(String, String)]) -> String {
    if context.is_empty() {
        return String::new();
    }

    context
        .iter()
        .map(|(span_name, fields)| format!("[{}: {}]", span_name, fields))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Formats span context with a custom formatter function.
///
/// This allows you to customize how each individual span is formatted.
///
/// # Arguments
///
/// * `context` - Slice of (span_name, fields) tuples
/// * `formatter` - Function that takes (span_name, fields) and returns a formatted string
///
/// # Returns
///
/// A formatted string with custom-formatted spans joined by spaces
///
/// # Examples
///
/// ```rust,ignore
/// use traccia::format_span_context_with;
///
/// let context = vec![("request".to_string(), "id=123".to_string())];
/// let span_str = format_span_context_with(&context, |name, fields| {
///     format!("{}({})", name, fields)
/// });
/// // Returns: "request(id=123)"
/// ```
pub fn format_span_context_with<F>(context: &[(String, String)], formatter: F) -> String
where
    F: Fn(&str, &str) -> String,
{
    if context.is_empty() {
        return String::new();
    }

    context
        .iter()
        .map(|(span_name, fields)| formatter(span_name, fields))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Helper builder for creating custom formatters with span positioning support.
///
/// This builder makes it easy to create custom formatters that support span positioning
/// without reimplementing the span positioning logic.
///
/// # Examples
///
/// ```rust,ignore
/// use traccia::{FormatterBuilder, SpanPosition};
///
/// // Create a formatter with timestamp and custom level format
/// let formatter = FormatterBuilder::new()
///     .with_span_position(SpanPosition::AfterLevel)
///     .build(|record, span_str| {
///         let timestamp = chrono::Local::now().format("%H:%M:%S");
///         let level = record.level.to_string().to_uppercase();
///
///         if span_str.is_empty() {
///             format!("[{}] {} {}", timestamp, level, record.message)
///         } else {
///             format!("[{}] {} {} {}", timestamp, level, span_str, record.message)
///         }
///     });
/// ```
pub struct FormatterBuilder {
    span_position: SpanPosition,
}

impl FormatterBuilder {
    /// Creates a new formatter builder with default span position (End).
    pub fn new() -> Self {
        Self {
            span_position: SpanPosition::End,
        }
    }

    /// Sets the span position for the formatter.
    pub fn with_span_position(mut self, position: SpanPosition) -> Self {
        self.span_position = position;
        self
    }

    /// Builds a custom formatter with the specified formatting function.
    ///
    /// The formatting function receives:
    /// - `record`: The log record to format
    /// - `span_str`: Pre-formatted span context string (may be empty)
    ///
    /// You are responsible for combining these elements in your desired format.
    ///
    /// # Arguments
    ///
    /// * `format_fn` - Function that takes (record, span_str) and returns a formatted string
    pub fn build<F>(self, format_fn: F) -> CustomFormatter<F>
    where
        F: Fn(&Record, &str) -> String + Send + Sync,
    {
        CustomFormatter {
            span_position: self.span_position,
            format_fn,
        }
    }
}

impl Default for FormatterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A custom formatter created by `FormatterBuilder`.
///
/// This formatter handles span positioning automatically while allowing
/// custom formatting logic for the rest of the log message.
pub struct CustomFormatter<F>
where
    F: Fn(&Record, &str) -> String + Send + Sync,
{
    span_position: SpanPosition,
    format_fn: F,
}

impl<F> Formatter for CustomFormatter<F>
where
    F: Fn(&Record, &str) -> String + Send + Sync,
{
    fn format(&self, record: &Record) -> String {
        let span_str = format_span_context(&record.context);

        match self.span_position {
            SpanPosition::None => {
                // Don't pass span string to format function
                (self.format_fn)(record, "")
            }
            _ => {
                // Pass span string and let format function handle positioning
                (self.format_fn)(record, &span_str)
            }
        }
    }
}

/// Helper function to format a record with span context at the specified position.
///
/// This utility function implements the standard span positioning logic.
/// Use this in custom formatters to get consistent span positioning behavior.
///
/// # Arguments
///
/// * `record` - The log record to format
/// * `position` - Where to place the span context
///
/// # Returns
///
/// A formatted string with the level, message, and span positioned according to `position`
///
/// # Examples
///
/// ```rust,ignore
/// use traccia::{Formatter, Record, SpanPosition, format_with_span_position};
///
/// struct MyFormatter;
///
/// impl Formatter for MyFormatter {
///     fn format(&self, record: &Record) -> String {
///         // Use the helper to get standard formatting with spans after level
///         format_with_span_position(record, SpanPosition::AfterLevel)
///     }
/// }
/// ```
pub fn format_with_span_position(record: &Record, position: SpanPosition) -> String {
    let level_str = format!("[{}]", record.level.default_coloring());
    let span_str = format_span_context(&record.context);

    match position {
        SpanPosition::End => {
            if span_str.is_empty() {
                format!("{} {}", level_str, record.message)
            } else {
                format!("{} {} {}", level_str, record.message, span_str)
            }
        }
        SpanPosition::Start => {
            if span_str.is_empty() {
                format!("{} {}", level_str, record.message)
            } else {
                format!("{} {} {}", span_str, level_str, record.message)
            }
        }
        SpanPosition::AfterLevel => {
            if span_str.is_empty() {
                format!("{} {}", level_str, record.message)
            } else {
                format!("{} {} {}", level_str, span_str, record.message)
            }
        }
        SpanPosition::None => {
            format!("{} {}", level_str, record.message)
        }
    }
}
