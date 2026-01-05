//! Span-based context management for structured logging.
//!
//! This module provides a way to add contextual information to log messages
//! through spans. Spans represent a period of execution and can be nested,
//! creating a hierarchy of context that is automatically included in all
//! log messages within that span.
//!
//! # Examples
//!
//! ```rust,ignore
//! use traccia::{span, info};
//!
//! fn process_request(user_id: u64) {
//!     let _span = span!("request", "user_id" => user_id);
//!
//!     info!("Processing request");
//!     // This will log: [INFO] Processing request [request: user_id=42]
//!
//!     process_data();
//! }
//!
//! fn process_data() {
//!     let _span = span!("data_processing", "stage" => "validation");
//!
//!     info!("Validating data");
//!     // This will log: [INFO] Validating data [request: user_id=42] [data_processing: stage=validation]
//! }
//! ```

use std::cell::RefCell;

/// Represents a single span with a name and key-value pairs.
#[derive(Debug, Clone)]
pub struct Span {
    /// The name of the span (e.g., "request", "database_query").
    pub name: String,

    /// Key-value pairs of context information.
    pub fields: Vec<(String, String)>,
}

impl Span {
    /// Creates a new span with the given name and fields.
    pub fn new(name: impl Into<String>, fields: Vec<(String, String)>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }
}

thread_local! {
    /// Thread-local stack of active spans.
    static SPAN_STACK: RefCell<Vec<Span>> = const { RefCell::new(Vec::new()) };
}

/// A guard that represents an active span.
///
/// When the guard is dropped, the span is automatically removed from the context.
/// This ensures proper cleanup even in the presence of early returns or panics.
pub struct SpanGuard {
    _private: (),
}

impl SpanGuard {
    /// Creates a new span guard and pushes the span onto the stack.
    pub fn new(span: Span) -> Self {
        SPAN_STACK.with(|stack| {
            stack.borrow_mut().push(span);
        });

        Self { _private: () }
    }
}

impl Drop for SpanGuard {
    fn drop(&mut self) {
        SPAN_STACK.with(|stack| {
            stack.borrow_mut().pop();
        });
    }
}

/// Returns the current span context as a vector of (span_name, key=value) pairs.
///
/// This function collects all active spans from the thread-local stack and
/// formats them for inclusion in log records.
pub fn current_context() -> Vec<(String, String)> {
    SPAN_STACK.with(|stack| {
        let stack = stack.borrow();
        let mut context = Vec::new();

        for span in stack.iter() {
            for (key, value) in &span.fields {
                context.push((span.name.clone(), format!("{}={}", key, value)));
            }
        }

        context
    })
}

/// Enters a new span with the given name and fields.
///
/// Returns a `SpanGuard` that will automatically exit the span when dropped.
/// This is the primary way to create spans programmatically.
pub fn enter(name: impl Into<String>, fields: Vec<(String, String)>) -> SpanGuard {
    SpanGuard::new(Span::new(name, fields))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_stack() {
        // Initially empty
        assert_eq!(current_context().len(), 0);

        {
            let _span1 = enter("test", vec![("key1".to_string(), "value1".to_string())]);
            let ctx = current_context();
            assert_eq!(ctx.len(), 1);
            assert_eq!(ctx[0].0, "test");
            assert_eq!(ctx[0].1, "key1=value1");

            {
                let _span2 = enter("nested", vec![("key2".to_string(), "value2".to_string())]);
                let ctx = current_context();
                assert_eq!(ctx.len(), 2);
                assert_eq!(ctx[1].0, "nested");
            }

            // After span2 drops
            let ctx = current_context();
            assert_eq!(ctx.len(), 1);
        }

        // After span1 drops
        assert_eq!(current_context().len(), 0);
    }

    #[test]
    fn test_multiple_fields() {
        let _span = enter(
            "multi",
            vec![
                ("field1".to_string(), "value1".to_string()),
                ("field2".to_string(), "value2".to_string()),
            ],
        );

        let ctx = current_context();
        assert_eq!(ctx.len(), 2);
        assert_eq!(ctx[0].1, "field1=value1");
        assert_eq!(ctx[1].1, "field2=value2");
    }

    #[test]
    fn test_span_with_logging() {
        use crate::{LogLevel, info, init};

        // This test verifies that spans integrate properly with the logging system
        // We can't easily verify the output, but we can ensure it doesn't panic
        init(LogLevel::Info);

        let _span = enter("test_log", vec![("key".to_string(), "value".to_string())]);

        // This should include the span context
        info!("Test message with span");

        {
            let _nested = enter(
                "nested",
                vec![("nested_key".to_string(), "nested_value".to_string())],
            );
            info!("Nested message");
        }

        info!("Back to outer span");
    }
}
