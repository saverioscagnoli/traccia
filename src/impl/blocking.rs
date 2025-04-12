use crate::{Config, DefaultFormatter, Formatter, Logger, Record, hooks};

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

        // Acquire the hook system lock
        // This is a read lock, so it won't block other threads from reading
        // but will block if another thread is writing
        // So, it fails only if the user tries to set a hook while the logger is running,
        // which is not encouraged.
        let hook_system = hooks::hook_system().read().expect(
            "Failed to acquire the hook system lock. You should use `set_hook` before initializing the logger.",
        );

        for target in &self.config.targets {
            // Check if the target has a custom filter level
            if let Some(filter_level) = target.filter_level() {
                if record.level < filter_level {
                    continue;
                }
            }

            let target_id = target.id();

            hook_system.trigger_before_log(record.level, &target_id);

            if let Err(e) = target.write(record.level, &formatted) {
                eprintln!("Failed to write to target: {}", e);
            }

            hook_system.trigger_after_log(record.level, &target_id);
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
