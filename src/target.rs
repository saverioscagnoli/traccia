use crate::{LogLevel, defaults::ConsoleFormatter, formatter::Formatter};

pub trait Target {
    /// Holds the formatted message
    fn write(&self, level: LogLevel, message: String) -> Result<(), Box<dyn std::error::Error>>;

    /// Optional method to override the global formatter
    fn formatter(&self) -> Option<Box<dyn Formatter>> {
        None
    }
}

pub struct Console;

impl Target for Console {
    fn write(&self, level: LogLevel, message: String) -> Result<(), Box<dyn std::error::Error>> {
        match level {
            LogLevel::Error | LogLevel::Fatal => eprintln!("{}", message),
            _ => println!("{}", message),
        }

        Ok(())
    }

    fn formatter(&self) -> Option<Box<dyn Formatter>> {
        Some(Box::new(ConsoleFormatter))
    }
}

impl Console {
    pub fn new() -> Self {
        Self
    }
}
