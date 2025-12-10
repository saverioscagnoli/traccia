//! Example demonstrating span-based context management.
//!
//! This example shows how to use spans to add contextual information
//! to log messages. Spans can be nested to create a hierarchy of context.

use traccia::{LogLevel, info, init, span, warn};

fn main() {
    // Initialize the logger
    init(LogLevel::Info);

    info!("Application started");

    // Example 1: Simple span with single field
    simple_span_example();

    // Example 2: Span with multiple fields
    multi_field_example();

    // Example 3: Nested spans
    nested_spans_example();

    // Example 4: Real-world scenario - HTTP request handling
    simulate_http_request("user123", "/api/users/42");
    simulate_http_request("user456", "/api/posts/10");

    info!("Application finished");
}

/// Example of a simple span with a single field
fn simple_span_example() {
    let _span = span!("simple_example", "operation" => "demo");

    info!("This log includes simple span context");
    warn!("Warnings also include the context");
}

/// Example of a span with multiple fields
fn multi_field_example() {
    let user_id = 12345;
    let session_id = "abc-def-ghi";

    let _span = span!("multi_field",
        "user_id" => user_id,
        "session_id" => session_id,
        "environment" => "production"
    );

    info!("Processing with multiple context fields");
}

/// Example of nested spans creating a context hierarchy
fn nested_spans_example() {
    let _outer = span!("outer_operation", "step" => 1);

    info!("In outer span");

    {
        let _inner = span!("inner_operation", "step" => 2);
        info!("In nested span - both contexts are included");

        {
            let _deepest = span!("deepest_operation", "step" => 3);
            info!("In deeply nested span - all three contexts are included");
        }

        info!("Back to inner span - only outer and inner contexts");
    }

    info!("Back to outer span - only outer context");
}

/// Simulates handling an HTTP request with nested operations
fn simulate_http_request(user_id: &str, path: &str) {
    // Create a request span that will be active for the entire request
    let _request_span = span!("http_request",
        "user_id" => user_id,
        "path" => path,
        "method" => "GET"
    );

    info!("Received request");

    // Authenticate the user
    authenticate_user(user_id);

    // Process the request
    process_request(path);

    info!("Request completed successfully");
}

fn authenticate_user(user_id: &str) {
    let _auth_span = span!("authentication", "user" => user_id);

    info!("Checking authentication");

    // Simulate database lookup
    check_database("auth_tokens", user_id);

    info!("Authentication successful");
}

fn process_request(path: &str) {
    let _process_span = span!("request_processing", "resource" => path);

    info!("Processing resource");

    // Simulate some processing steps
    if path.contains("/users/") {
        fetch_user_data();
    } else if path.contains("/posts/") {
        fetch_post_data();
    }

    info!("Resource processing complete");
}

fn check_database(table: &str, key: &str) {
    let _db_span = span!("database_query",
        "table" => table,
        "key" => key,
        "operation" => "SELECT"
    );

    info!("Executing database query");
    // Simulate query time
    std::thread::sleep(std::time::Duration::from_millis(10));
    info!("Query completed");
}

fn fetch_user_data() {
    let _span = span!("fetch_user", "cache" => "miss");
    info!("Fetching user from database");
    check_database("users", "42");
}

fn fetch_post_data() {
    let _span = span!("fetch_post", "cache" => "hit");
    info!("Fetching post from cache");
}
