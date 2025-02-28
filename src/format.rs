use crate::LogLevel;

pub(crate) fn default_format(level: LogLevel, message: &str) -> String {
    format!("[{}] {}\n", level, message)
}
