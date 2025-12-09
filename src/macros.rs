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
            };

            use $crate::Logger;
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
