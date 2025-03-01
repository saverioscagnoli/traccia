use crate::{Color, Colorize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

impl LogLevel {
    pub fn default_coloring(&self) -> String {
        match self {
            LogLevel::Trace => format!("{}", self).color(Color::Cyan),
            LogLevel::Debug => format!("{}", self).color(Color::Blue),
            LogLevel::Info => format!("{}", self).color(Color::Green),
            LogLevel::Warn => format!("{}", self).color(Color::Yellow),
            LogLevel::Error => format!("{}", self).color(Color::Red),
        }
    }
}
