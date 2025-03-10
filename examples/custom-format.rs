use traccia::{debug, error, fatal, info, trace, warn, LogLevel};

struct CustomFormatter;

impl traccia::Formatter for CustomFormatter {
    fn format(&self, record: &traccia::Record) -> String {
        format!(
            ":D [{}] [{}:{}] {}",
            record.level.default_coloring(),
            record.file.as_ref().unwrap(),
            record.line.unwrap(),
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
