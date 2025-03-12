use std::{
    sync::{Mutex, mpsc},
    thread,
};

use crate::{Config, DefaultFormatter, Formatter, LogLevel, Logger, Record, Target};

enum ChannelMessage {
    Log(String, LogLevel),
    Flush,
}

pub struct DefaultLogger {
    config: Config,
    sender: mpsc::Sender<ChannelMessage>,
    worker: Mutex<Option<thread::JoinHandle<()>>>,
}

impl DefaultLogger {
    pub fn new(config: Config) -> Self {
        let (sender, receiver) = mpsc::channel();

        let thread_targerts = config.targets.clone();
        let worker = std::thread::spawn(move || {
            Self::worker_thread(receiver, thread_targerts);
        });

        DefaultLogger {
            config,
            sender,
            worker: Mutex::new(Some(worker)),
        }
    }

    fn process_message(formatted: &str, level: LogLevel, targets: &[Box<dyn Target>]) {
        for target in targets {
            // Check if the target has a custom filter level
            if let Some(filter_level) = target.filter_level() {
                if level < filter_level {
                    continue;
                }
            }

            if let Err(e) = target.write(&formatted) {
                eprintln!("Failed to write to target: {}", e);
            }
        }
    }

    fn worker_thread(receiver: mpsc::Receiver<ChannelMessage>, targets: Vec<Box<dyn Target>>) {
        loop {
            match receiver.recv() {
                Ok(ChannelMessage::Log(formatted, level)) => {
                    Self::process_message(&formatted, level, &targets)
                }

                Ok(ChannelMessage::Flush) => break,

                Err(_) => break,
            }
        }

        // Drain the remaining messages
        while let Ok(message) = receiver.try_recv() {
            match message {
                ChannelMessage::Log(formatted, level) => {
                    Self::process_message(&formatted, level, &targets)
                }

                _ => {}
            }
        }
    }
}

impl Logger for DefaultLogger {
    fn enabled(&self, level: crate::LogLevel) -> bool {
        self.config.level <= level
    }

    fn abort(&self) {
        let _ = self.sender.send(ChannelMessage::Flush);
        if let Ok(mut handle) = self.worker.lock() {
            if let Some(handle) = handle.take() {
                handle.join().unwrap();
            }

            eprintln!("Cleanup process failed. Some final logs may not be written.");
        }
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.level) {
            return;
        }

        let formatted = match &self.config.format {
            Some(formatter) => formatter.format(record),
            None => DefaultFormatter.format(record),
        };

        let _ = self
            .sender
            .send(ChannelMessage::Log(formatted, record.level));
    }
}

impl Default for DefaultLogger {
    fn default() -> Self {
        DefaultLogger::new(Config::default())
    }
}
