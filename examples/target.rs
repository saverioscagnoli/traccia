use traccia::{LogLevel, debug, error, info, trace, warn};

fn main() {
    traccia::init_with_config(traccia::Config {
        level: LogLevel::Trace,
        targets: vec![
            Box::new(traccia::Console),
            Box::new(traccia::File::new(".logs/latest.log").unwrap()),
        ],
        ..Default::default()
    });

    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warn message");
    error!("This is an error message");

    // If not using the blocking feature, call `flush` function to flush the log buffer
    #[cfg(not(feature = "blocking"))]
    traccia::flush();
}
