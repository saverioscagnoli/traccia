//! Example demonstrating span context after the log level.
//!
//! This example shows how to configure the logger to place span context
//! after the log level but before the message.

use traccia::{
    Config, Console, DefaultFormatter, LogLevel, debug, info, init_with_config, span, warn,
};

fn main() {
    // Configure logger with span context after the log level
    let config = Config {
        level: LogLevel::Debug,
        targets: vec![Box::new(Console::new())],
        format: Some(Box::new(DefaultFormatter::with_span_after_level())),
    };

    init_with_config(config);

    info!("Application started");

    // Simulate a web service handling requests
    handle_api_request("GET", "/api/users/123", "192.168.1.100");
    handle_api_request("POST", "/api/posts", "192.168.1.101");

    info!("Application finished");
}

fn handle_api_request(method: &str, endpoint: &str, client_ip: &str) {
    let _request = span!("http_request",
        "method" => method,
        "endpoint" => endpoint,
        "client_ip" => client_ip
    );

    info!("Incoming request");
    debug!("Parsing request headers");

    validate_request(endpoint);
    execute_handler(endpoint);

    info!("Request handled successfully");
}

fn validate_request(endpoint: &str) {
    let _validation = span!("validation", "stage" => "request");

    debug!("Starting validation");

    if endpoint.contains("/api/") {
        info!("API endpoint validated");
    } else {
        warn!("Non-API endpoint detected");
    }

    debug!("Validation complete");
}

fn execute_handler(endpoint: &str) {
    let _handler = span!("handler", "resource" => endpoint);

    debug!("Executing handler");

    if endpoint.contains("/users/") {
        fetch_user_data();
    } else if endpoint.contains("/posts") {
        fetch_posts_data();
    }

    debug!("Handler execution complete");
}

fn fetch_user_data() {
    let _db = span!("database",
        "query" => "SELECT",
        "table" => "users"
    );

    debug!("Connecting to database");
    info!("Fetching user data");
    debug!("Query executed successfully");
}

fn fetch_posts_data() {
    let _db = span!("database",
        "query" => "SELECT",
        "table" => "posts"
    );

    debug!("Connecting to database");
    info!("Fetching posts data");
    debug!("Query executed successfully");
}
