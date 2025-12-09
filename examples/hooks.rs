use traccia::{Hook, LogLevel, TargetId, info, warn};

fn main() {
    traccia::set_hook(Hook::AfterLog(Box::new(|_, target| {
        if let TargetId::Console(_) = target {
            println!("This will be printed after the log message");
        }
    })));

    traccia::set_hook(Hook::BeforeLog(Box::new(|level, target| {
        if let TargetId::File(_) = target {
            if level == LogLevel::Info {
                println!("This will be printed only before calling the info! macro on a file.")
            }
        }
    })));

    traccia::set_hook(Hook::BeforeLog(Box::new(|_, target| {
        if let TargetId::Console(_) = target {
            println!("This will be printed before the log message");
        }
    })));

    traccia::init_with_config(traccia::Config {
        level: LogLevel::Trace,
        targets: vec![
            Box::new(traccia::Console::new()),
            Box::new(
                traccia::File::new("./.logs/latest.log", traccia::FileMode::Truncate)
                    .expect("Failed to open file."),
            ),
        ],
        ..Default::default()
    });

    info!("This is a test log message");
    warn!("This is a test warning message");
}
