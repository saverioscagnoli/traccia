//! Example demonstrating span context at the start of log messages.
//!
//! This example shows how to configure the logger to place span context
//! at the beginning of each log message, before the log level.

use traccia::{Config, Console, DefaultFormatter, LogLevel, info, init_with_config, span, warn};

fn main() {
    // Configure logger with span context at the start
    let config = Config {
        level: LogLevel::Info,
        targets: vec![Box::new(Console::new())],
        format: Some(Box::new(DefaultFormatter::with_span_at_start())),
    };

    init_with_config(config);

    info!("Application started");

    // Simple span
    process_request(12345, "alice");

    // Another request
    process_request(67890, "bob");

    info!("Application finished");
}

fn process_request(request_id: u64, user: &str) {
    let _request = span!("request",
        "id" => request_id,
        "user" => user
    );

    info!("Processing request");

    authenticate(user);
    fetch_data(request_id);

    info!("Request completed");
}

fn authenticate(user: &str) {
    let _auth = span!("auth", "user" => user);

    info!("Authenticating user");
    warn!("Using legacy authentication method");
}

fn fetch_data(id: u64) {
    let _fetch = span!("database", "operation" => "SELECT", "id" => id);

    info!("Fetching data from database");
    info!("Data retrieved successfully");
}
