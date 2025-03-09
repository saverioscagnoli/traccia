use traccia::{FileMode, LogLevel, debug, error, fatal, info, trace, warn};

fn main() {
    traccia::init_with_config(traccia::Config {
        level: LogLevel::Trace,
        targets: vec![
            Box::new(traccia::Console::new()),
            Box::new(
                traccia::File::new_filtered(
                    ".logs/latest.log",
                    FileMode::Truncate,
                    LogLevel::Fatal,
                )
                .unwrap(),
            ),
        ],
        ..Default::default()
    });

    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warn message");
    error!("This is an error message");
    fatal!("This is a fatal message");
}
