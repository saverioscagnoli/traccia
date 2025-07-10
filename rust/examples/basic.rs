use traccia::{LogLevel, debug, error, fatal, info, trace, warn};

fn main() {
    traccia::init(LogLevel::Debug);

    // This will not be logged if the log level is set to debug
    trace!("This is a trace message");

    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warn message");
    error!("This is an error message");
    fatal!("This is a fatal message");
}
