use clap::Parser;
use traccia::{LogLevel, debug, fatal, info};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The log level to use
    #[arg(short, long, default_value_t = LogLevel::Info)]
    level: LogLevel,
}

fn main() {
    let args = Args::parse();

    traccia::init(args.level);

    debug!("This will not be logged if no argument is specified.");
    info!("This will be logged if no argument is specified.");

    fatal!("Maybe pass -l fatal for only fatal messages to be logged?");
}
