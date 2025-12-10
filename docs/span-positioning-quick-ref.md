# Span Positioning Quick Reference

## TL;DR

Use `ConfigurableFormatter` to control where span context appears in your logs.

```rust
use traccia::{Config, DefaultFormatter, Console, LogLevel, init_with_config};

let config = Config {
    level: LogLevel::Info,
    targets: vec![Box::new(Console::new())],
    format: Some(Box::new(DefaultFormatter::with_span_at_start())),
};

init_with_config(config);
```

## Four Positions Available

| Position | Convenience Method | Output Format |
|----------|-------------------|---------------|
| **End** (default) | `with_span_at_end()` | `[INFO] message [span: key=value]` |
| **Start** | `with_span_at_start()` | `[span: key=value] [INFO] message` |
| **After Level** | `with_span_after_level()` | `[INFO] [span: key=value] message` |
| **None** | `without_span()` | `[INFO] message` |

## Visual Comparison

Given this code:
```rust
let _request = span!("request", "id" => 42);
let _db = span!("database", "table" => "users");
info!("Query executed");
```

### Position: End (Default)
```
[INFO] Query executed [request: id=42] [database: table=users]
```

### Position: Start
```
[request: id=42] [database: table=users] [INFO] Query executed
```

### Position: AfterLevel
```
[INFO] [request: id=42] [database: table=users] Query executed
```

### Position: None
```
[INFO] Query executed
```

## Complete Examples

### Using Convenience Methods

```rust
use traccia::{Config, DefaultFormatter, Console, LogLevel, init_with_config};

// Span at start
let config = Config {
    level: LogLevel::Info,
    targets: vec![Box::new(Console::new())],
    format: Some(Box::new(DefaultFormatter::with_span_at_start())),
};
init_with_config(config);
```

### Using SpanPosition Enum

```rust
use traccia::{Config, DefaultFormatter, Console, LogLevel, SpanPosition, init_with_config};

let config = Config {
    level: LogLevel::Info,
    targets: vec![Box::new(Console::new())],
    format: Some(Box::new(DefaultFormatter::with_position(SpanPosition::AfterLevel))),
};
init_with_config(config);
```

## When to Use Each Position

### End (Default) ✅ Most Readable
**Use when:**
- Human readability is the priority
- Logs are primarily read directly by developers
- Message content is more important than context

**Example use case:** Development environments, debugging sessions

### Start ✅ Best for Parsing
**Use when:**
- Logs are consumed by aggregation systems
- You need structured data at the beginning
- Context-based filtering is important

**Example use case:** Production environments with log aggregation (Elasticsearch, Splunk)

### AfterLevel ✅ Balanced
**Use when:**
- You want both readability and parseability
- Log level should be first for quick scanning
- Context should be easy to extract programmatically

**Example use case:** General purpose logging with both human and machine consumers

### None ✅ Clean Output
**Use when:**
- You want spans in code but not in output
- Testing or specific environments need clean logs
- Performance is critical and span output is unnecessary

**Example use case:** CI/CD pipelines, minimal output modes

## Code Snippets

### Span at Start
```rust
use traccia::{Config, DefaultFormatter, Console, LogLevel, init_with_config, span, info};

let config = Config {
    level: LogLevel::Info,
    targets: vec![Box::new(Console::new())],
    format: Some(Box::new(DefaultFormatter::with_span_at_start())),
};
init_with_config(config);

let _span = span!("api", "endpoint" => "/users");
info!("Request received");
// Output: [api: endpoint=/users] [INFO] Request received
```

### Span After Level
```rust
use traccia::{Config, DefaultFormatter, Console, LogLevel, init_with_config, span, info};

let config = Config {
    level: LogLevel::Info,
    targets: vec![Box::new(Console::new())],
    format: Some(Box::new(DefaultFormatter::with_span_after_level())),
};
init_with_config(config);

let _span = span!("api", "endpoint" => "/users");
info!("Request received");
// Output: [INFO] [api: endpoint=/users] Request received
```

### No Span Context
```rust
use traccia::{Config, DefaultFormatter, Console, LogLevel, init_with_config, span, info};

let config = Config {
    level: LogLevel::Info,
    targets: vec![Box::new(Console::new())],
    format: Some(Box::new(DefaultFormatter::without_span())),
};
init_with_config(config);

let _span = span!("api", "endpoint" => "/users");
info!("Request received");
// Output: [INFO] Request received
```

## All Available Methods

```rust
use traccia::{DefaultFormatter, SpanPosition};

// Convenience constructors
let f1 = DefaultFormatter::with_span_at_start();
let f2 = DefaultFormatter::with_span_after_level();
let f3 = DefaultFormatter::with_span_at_end();
let f4 = DefaultFormatter::without_span();

// Using SpanPosition enum
let f5 = DefaultFormatter::with_position(SpanPosition::Start);
let f6 = DefaultFormatter::with_position(SpanPosition::AfterLevel);
let f7 = DefaultFormatter::with_position(SpanPosition::End);
let f8 = DefaultFormatter::with_position(SpanPosition::None);

// Default is SpanPosition::End
let f9 = DefaultFormatter::default();
```

## See Also

- [Full Spans Documentation](./spans.md)
- [Custom Formatter Guide](../README.md#custom-formatters)
- Examples: `examples/span-at-start.rs`, `examples/span-after-level.rs`
