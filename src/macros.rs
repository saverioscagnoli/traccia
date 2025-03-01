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
/// error!("Failed to connect: {}", err);
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Error, $($arg)*)
    };
}
