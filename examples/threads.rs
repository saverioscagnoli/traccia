use traccia::{LogLevel, Record};

struct CustomFormatter;

impl traccia::Formatter for CustomFormatter {
    fn format(&self, record: &Record) -> String {
        let id_str = format!("{:?}", record.thread_id);
        let id_str = id_str.replace("ThreadId(", "").replace(")", "");

        format!(
            "[{}] [thread:{}] {}",
            record.level.default_coloring(),
            id_str,
            record.message,
        )
    }
}

fn main() {
    traccia::init_with_config(traccia::Config {
        level: LogLevel::Trace,
        format: Some(Box::new(CustomFormatter)),
        ..Default::default()
    });

    let handles = (0..3)
        .map(|i| {
            std::thread::spawn(move || {
                traccia::trace!("This is a trace message from thread {}", i);
                traccia::debug!("This is a debug message from thread {}", i);
                traccia::info!("This is an info message from thread {}", i);
                traccia::warn!("This is a warn message from thread {}", i);
                traccia::error!("This is an error message from thread {}", i);
                traccia::fatal!("This is a fatal message from thread {}", i);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
}
