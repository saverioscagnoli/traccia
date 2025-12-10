# Span-Based Context Management

Spans provide a powerful way to add contextual information to your log messages. They represent a period of execution and can be nested to create a hierarchy of context that is automatically included in all log messages within that span.

## Overview

A span is a named scope with associated key-value pairs that automatically adds context to every log message within that scope. When a span guard is dropped (goes out of scope), the span is automatically removed from the context stack.

## Basic Usage

### Creating a Simple Span

```rust
use traccia::{span, info, init_default};

fn main() {
    init_default();
    
    let _span = span!("operation_name", "key" => "value");
    
    info!("This log includes the span context");
    // Output: [INFO] This log includes the span context [operation_name: key=value]
}
```

### Multiple Fields

You can add multiple key-value pairs to a span:

```rust
let user_id = 12345;
let session_id = "abc-def-ghi";

let _span = span!("request",
    "user_id" => user_id,
    "session_id" => session_id,
    "environment" => "production"
);

info!("Processing request");
// Output: [INFO] Processing request [request: user_id=12345] [request: session_id=abc-def-ghi] [request: environment=production]
```

### Nested Spans

Spans can be nested to create hierarchical context:

```rust
fn process_request() {
    let _outer = span!("request", "id" => 123);
    
    info!("Processing request");
    // Output: [INFO] Processing request [request: id=123]
    
    {
        let _inner = span!("validation", "stage" => "input");
        
        info!("Validating input");
        // Output: [INFO] Validating input [request: id=123] [validation: stage=input]
    }
    
    info!("Validation complete");
    // Output: [INFO] Validation complete [request: id=123]
}
```

## Real-World Example: HTTP Request Handling

```rust
use traccia::{span, info, init_default};

fn main() {
    init_default();
    handle_request("user123", "/api/users/42", "GET");
}

fn handle_request(user_id: &str, path: &str, method: &str) {
    let _request_span = span!("http_request",
        "user_id" => user_id,
        "path" => path,
        "method" => method
    );
    
    info!("Received request");
    
    authenticate(user_id);
    fetch_data(path);
    
    info!("Request completed");
}

fn authenticate(user_id: &str) {
    let _auth = span!("auth", "user" => user_id);
    
    info!("Checking credentials");
    // All logs here will include both http_request and auth context
}

fn fetch_data(path: &str) {
    let _fetch = span!("data_fetch", "resource" => path);
    
    info!("Fetching from database");
    // All logs here will include both http_request and data_fetch context
}
```

Output:
```
[INFO] Received request [http_request: user_id=user123] [http_request: path=/api/users/42] [http_request: method=GET]
[INFO] Checking credentials [http_request: user_id=user123] [http_request: path=/api/users/42] [http_request: method=GET] [auth: user=user123]
[INFO] Fetching from database [http_request: user_id=user123] [http_request: path=/api/users/42] [http_request: method=GET] [data_fetch: resource=/api/users/42]
[INFO] Request completed [http_request: user_id=user123] [http_request: path=/api/users/42] [http_request: method=GET]
```

## Customizing Span Position

By default, span context appears at the end of log messages, but you can customize where it appears using the `DefaultFormatter`.

### Available Positions

The `SpanPosition` enum provides four options:

- **`SpanPosition::End`** (default) - Context appears at the end
  - Format: `[INFO] message [span: key=value]`
- **`SpanPosition::Start`** - Context appears before the log level
  - Format: `[span: key=value] [INFO] message`
- **`SpanPosition::AfterLevel`** - Context appears after level, before message
  - Format: `[INFO] [span: key=value] message`
- **`SpanPosition::None`** - Context is not included in output
  - Format: `[INFO] message`

### Usage Examples

#### Span at the Start

```rust
use traccia::{Config, DefaultFormatter, Console, LogLevel, init_with_config, span, info};

let config = Config {
    level: LogLevel::Info,
    targets: vec![Box::new(Console::new())],
    format: Some(Box::new(DefaultFormatter::with_span_at_start())),
};

init_with_config(config);

let _span = span!("request", "id" => 123);
info!("Processing");
// Output: [request: id=123] [INFO] Processing
```

#### Span After Level

```rust
use traccia::{Config, DefaultFormatter, Console, LogLevel, init_with_config};

let config = Config {
    level: LogLevel::Info,
    targets: vec![Box::new(Console::new())],
    format: Some(Box::new(DefaultFormatter::with_span_after_level())),
};

init_with_config(config);

let _span = span!("auth", "user" => "alice");
info!("Authenticated");
// Output: [INFO] [auth: user=alice] Authenticated
```

#### Disable Span Context

```rust
use traccia::{Config, DefaultFormatter, Console, LogLevel, init_with_config};

let config = Config {
    level: LogLevel::Info,
    targets: vec![Box::new(Console::new())],
    format: Some(Box::new(DefaultFormatter::without_span())),
};

init_with_config(config);

let _span = span!("test", "key" => "value");
info!("Message");
// Output: [INFO] Message (no span context)
```

#### Using SpanPosition Directly

```rust
use traccia::{DefaultFormatter, SpanPosition};

// These are equivalent ways to create formatters:
let formatter1 = DefaultFormatter::with_span_at_start();
let formatter2 = DefaultFormatter::with_position(SpanPosition::Start);

let formatter3 = DefaultFormatter::with_span_after_level();
let formatter4 = DefaultFormatter::with_position(SpanPosition::AfterLevel);
```

## Best Practices

### 1. Use Descriptive Span Names

Choose span names that clearly indicate what operation or scope they represent:

```rust
// Good
let _span = span!("database_query", "table" => "users");
let _span = span!("http_request", "endpoint" => "/api/v1/users");

// Less clear
let _span = span!("db", "t" => "users");
let _span = span!("req", "e" => "/api/v1/users");
```

### 2. Always Bind the Span Guard

Always assign the span guard to a variable (even if unused with `_`). If you don't bind it, it will be dropped immediately:

```rust
// ✓ Correct - span is active for the entire scope
let _span = span!("operation", "id" => 123);
info!("Log with context");

// ✗ Wrong - span is immediately dropped
span!("operation", "id" => 123);
info!("Log WITHOUT context");
```

### 3. Keep Spans Short-Lived

Spans should represent a specific operation or scope. Don't create spans that live for the entire application lifetime:

```rust
// Good - span for a specific operation
fn process_item(id: u64) {
    let _span = span!("item_processing", "id" => id);
    // processing logic
}

// Less ideal - span lives too long
fn main() {
    let _span = span!("application", "version" => "1.0");
    // entire application runs in this span
}
```

### 4. Use Meaningful Key-Value Pairs

Include information that helps with debugging and tracing:

```rust
let _span = span!("api_call",
    "endpoint" => endpoint,
    "method" => "POST",
    "retry_count" => retry_count,
    "timeout_ms" => timeout
);
```

## Thread Safety

Spans are stored in thread-local storage, meaning each thread has its own independent span stack. This makes spans thread-safe and prevents cross-thread context pollution:

```rust
use std::thread;
use traccia::{span, info};

let _main_span = span!("main_thread", "id" => "main");
info!("In main thread");

let handle = thread::spawn(|| {
    let _worker_span = span!("worker_thread", "id" => "worker");
    info!("In worker thread");
    // This log only has worker_thread context, not main_thread
});

handle.join().unwrap();
info!("Back in main thread");
// This log only has main_thread context
```

## Integration with Custom Formatters

If you're using a custom formatter, you can access the span context through the `Record.context` field:

```rust
use traccia::{Formatter, Record};

struct MyFormatter;

impl Formatter for MyFormatter {
    fn format(&self, record: &Record) -> String {
        let mut output = format!("[{}] {}", record.level, record.message);
        
        // Access span context
        for (span_name, fields) in &record.context {
            output.push_str(&format!(" [{}: {}]", span_name, fields));
        }
        
        output
    }
}
```

The `DefaultFormatter` is also configurable, so you may not need a custom formatter just to change span positioning.

## API Reference

### Macros

- `span!(name)` - Creates a span with only a name
- `span!(name, key => value)` - Creates a span with one field
- `span!(name, k1 => v1, k2 => v2, ...)` - Creates a span with multiple fields

### Functions

- `traccia::enter(name, fields)` - Programmatically creates a span (used by the macro)
- `traccia::current_context()` - Returns the current span context as a vector

### Types

- `traccia::SpanGuard` - Guard that removes the span when dropped
- `traccia::Span` - Represents a span with name and fields

## Performance Considerations

Spans are designed to be lightweight:

- Thread-local storage is used for fast access
- Context is only collected when a log message is actually emitted
- The guard uses RAII for automatic cleanup
- No heap allocations unless fields contain owned strings

For hot paths where performance is critical, consider using spans at a higher level rather than for every tiny operation.

## Choosing the Right Span Position

Different positions work better for different use cases:

- **End** - Best for human readability when message is more important than context
- **Start** - Best for log aggregation systems that parse structured data from the beginning
- **AfterLevel** - Best balance between readability and parseability
- **None** - Use when you want spans in code but don't need them in output (e.g., development vs production)