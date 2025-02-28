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

            logger.log(&record);
        }
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Trace, $($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Debug, $($arg)*)
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Info, $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Warn, $($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Error, $($arg)*)
    };
}
