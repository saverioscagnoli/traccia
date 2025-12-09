use traccia::{LogLevel, debug, error, fatal, info, trace, warn};

struct CustomFormatter;

impl traccia::Formatter for CustomFormatter {
    fn format(&self, record: &traccia::Record) -> String {
        format!(
            "{}: {}",
            record.level.default_coloring().to_lowercase(),
            record.message
        )
    }
}

fn main() {
    traccia::init_with_config(traccia::Config {
        level: LogLevel::Trace,
        format: Some(Box::new(CustomFormatter)),
        ..Default::default()
    });

    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warn message");
    error!("This is an error message");
    fatal!("This is a fatal message");
}
