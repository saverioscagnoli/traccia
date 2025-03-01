use traccia::LogLevel;

fn main() {
    traccia::init(LogLevel::Trace);

    let handles = (0..3)
        .map(|i| {
            std::thread::spawn(move || {
                traccia::trace!("This is a trace message from thread {}", i);
                traccia::debug!("This is a debug message from thread {}", i);
                traccia::info!("This is an info message from thread {}", i);
                traccia::warn!("This is a warn message from thread {}", i);
                traccia::error!("This is an error message from thread {}", i);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    // If not using the blocking feature, call `flush` function to flush the log buffer
    #[cfg(not(feature = "blocking"))]
    traccia::flush();
}
