use traccia::{debug, error, fatal, info, Console, LogLevel, Output};

fn main() {
    traccia::init_with_config(traccia::Config {
        level: LogLevel::Debug,
        targets: vec![Box::new(
            Console::new()
                .filtered_output(LogLevel::Error, Output::Stderr)
                .filtered_output(LogLevel::Fatal, Output::Stderr),
        )],
        ..Default::default()
    });

    debug!("This will be logged to stdout");
    info!("In fact, only error and fatal messages will be logged to stderr.");

    error!("This is an error logged to stderr!!!");
    fatal!("This is a fatal error logged to stderr!!!");
}
