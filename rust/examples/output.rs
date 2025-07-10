use traccia::{debug, error, fatal, info, Console, LogLevel, Output};

fn main() {
    traccia::init_with_config(traccia::Config {
        level: LogLevel::Debug,
        targets: vec![Box::new(Console::new().output(Output::Stderr))],
        ..Default::default()
    });

    debug!("This will be logged to stderr.");
    info!("In fact, all messages will be logged to stderr.");
    error!("This is an error logged to stderr!!!");
    fatal!("This is a fatal error logged to stderr!!!");
}
