/// Macro for logging messages at a specific level.
///
/// This is the core logging macro that other macros (`debug!`, `info!`, etc.) build upon.
///
/// # Arguments
///
/// * `$level` - The log level to use
/// * `$arg` - Format string and arguments, similar to `format!` or `println!`
#[macro_export]
macro_rules! log {
   ($level:expr, $($arg:tt)*) => {{
        if let Ok(logger) = $crate::logger() {
            let record = $crate::Record {
                level: $level,
                thread_id: std::thread::current().id(),
                target: module_path!().to_string(),
                message: format!($($arg)*),
                module_path: Some(module_path!()),
                file: Some(file!()),
                line: Some(line!()),
                context: $crate::current_context(),
            };

            logger.log(&record);
        }
    }};
}

/// Logs a message at the TRACE level.
///
/// # Examples
///
/// ```
/// use traccia::{init_default, trace};
///
/// init_default();
/// let item_id = 42;
/// trace!("Processing item: {}", item_id);
/// ```
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Trace, $($arg)*)
    };
}

/// Logs a message at the DEBUG level.
///
/// # Examples
///
/// ```
/// use traccia::{init_default, debug};
///
/// init_default();
/// let conn_id = 123;
/// debug!("Connection established: {}", conn_id);
/// ```
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Debug, $($arg)*)
    };
}

/// Logs a message at the INFO level.
///
/// # Examples
///
/// ```
/// use traccia::{init_default, info};
///
/// init_default();
/// info!("Application started");
/// ```
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Info, $($arg)*)
    };
}

/// Logs a message at the WARN level.
///
/// # Examples
///
/// ```
/// use traccia::{init_default, warn};
///
/// init_default();
/// let usage = 85;
/// warn!("Resource usage high: {}%", usage);
/// ```
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Warn, $($arg)*)
    };
}

/// Logs a message at the ERROR level.
///
/// # Examples
///
/// ```
/// use traccia::{init_default, error};
///
/// init_default();
/// let err = "timeout";
/// error!("Failed to connect: {}", err);
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Error, $($arg)*)
    };
}

/// Logs a message at the FATAL level.
///
/// # Examples
///
/// ```
/// use traccia::{init_default, fatal};
///
/// init_default();
/// let err = "configuration error";
/// fatal!("Failed to start application: {}", err);
/// ```
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Fatal, $($arg)*)
    };
}

/// Creates a new span with the given name and fields.
///
/// The span is active until the returned guard is dropped. All log messages
/// within the span will include the span's context information.
///
/// # Syntax
///
/// ```ignore
/// span!(name)
/// span!(name, key1 => value1)
/// span!(name, key1 => value1, key2 => value2, ...)
/// ```
///
/// # Examples
///
/// ```
/// use traccia::{span, info, init_default};
///
/// init_default();
///
/// fn process_request(user_id: u64) {
///     let _span = span!("request", "user_id" => user_id);
///     info!("Processing request");
///     // Logs: [INFO] Processing request [request: user_id=42]
/// }
/// ```
///
/// ```
/// use traccia::{span, info, init_default};
///
/// init_default();
///
/// fn handle_connection(conn_id: u32, ip: &str) {
///     let _span = span!("connection", "id" => conn_id, "ip" => ip);
///     info!("Connection established");
///     // Logs: [INFO] Connection established [connection: id=123, ip=127.0.0.1]
/// }
/// ```
#[macro_export]
macro_rules! span {
    ($name:expr) => {
        $crate::enter($name, vec![])
    };
    ($name:expr, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::enter($name, vec![$(
            ($key.to_string(), $value.to_string())
        ),+])
    };
}
