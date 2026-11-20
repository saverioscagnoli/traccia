/// Returns `level`, raised to at least `Debug` when the *calling* crate is
/// built with `debug_assertions`. `cfg!` expands in the caller's crate, so
/// this follows the app's profile, not this crate's.
#[macro_export]
macro_rules! debug_or {
    ($level:expr) => {{
        let level: $crate::LevelFilter = $level;
        if cfg!(debug_assertions) && level < $crate::LevelFilter::Debug {
            $crate::LevelFilter::Debug
        } else {
            level
        }
    }};
}

#[macro_export]
macro_rules! debug_or_info {
    () => {
        $crate::debug_or!($crate::LevelFilter::Info)
    };
}

#[macro_export]
macro_rules! debug_or_warn {
    () => {
        $crate::debug_or!($crate::LevelFilter::Warn)
    };
}

#[macro_export]
macro_rules! debug_or_error {
    () => {
        $crate::debug_or!($crate::LevelFilter::Error)
    };
}
