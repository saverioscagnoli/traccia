use traccia::{LogLevel, debug, error, fatal, info, trace, warn};

fn main() {
    traccia::init_with_level(LogLevel::Trace);

    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warn message");
    error!("This is an error message");
    fatal!("This is a fatal message");
}
