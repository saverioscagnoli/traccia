use crate::{Config, DefaultFormatter, Formatter, Logger, Record};

pub struct DefaultLogger {
    config: Config,
}

impl DefaultLogger {
    pub fn new(config: Config) -> Self {
        DefaultLogger { config }
    }
}

impl Logger for DefaultLogger {
    fn enabled(&self, level: crate::LogLevel) -> bool {
        self.config.level <= level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.level) {
            return;
        }

        let formatted = match &self.config.format {
            Some(formatter) => formatter.format(record),
            None => DefaultFormatter.format(record),
        };

        for target in &self.config.targets {
            if let Some(filter_level) = target.filter_level() {
                if record.level < filter_level {
                    continue;
                }
            }

            if let Err(e) = target.write(record.level, &formatted) {
                eprintln!("Failed to write to target: {}", e);
            }
        }
    }
}

impl Default for DefaultLogger {
    fn default() -> Self {
        DefaultLogger {
            config: Config::default(),
        }
    }
}
