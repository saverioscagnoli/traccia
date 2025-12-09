use traccia::{FileMode, LogLevel, error, fatal, info};

fn main() {
    traccia::init_with_config(traccia::Config {
        level: LogLevel::Trace,
        targets: vec![
            Box::new(traccia::Console::new()),
            Box::new(
                traccia::File::new("./.logs/latest.log", FileMode::Truncate)
                    .expect("Failed to open file.")
                    .filtered(LogLevel::Fatal),
            ),
        ],
        ..Default::default()
    });

    info!("This will not be written to latest.log, but will be printed to console.");
    error!("It will write fatal messages only!");
    fatal!("Like this :(");
}
