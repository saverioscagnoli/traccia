use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use traccia::Config;
use traccia::Console;
use traccia::Level;
use traccia::LevelFilter;
use traccia::Target;
use traccia::debug;
use traccia::error;
use traccia::info;
use traccia::warn;

#[derive(Clone, Default)]
struct Memory {
    lines: Arc<Mutex<Vec<String>>>,
}

impl Target for Memory {
    fn write(&self, level: Level, message: &str) -> Result<(), Box<dyn Error>> {
        if level <= Level::Warn {
            self.lines.lock().unwrap().push(message.to_string());
        }

        Ok(())
    }
}

fn main() {
    let memory = Memory::default();

    traccia::init(
        Config::empty()
            .with_min_level(LevelFilter::Debug)
            .with_target(Console::default())
            .with_target(memory.clone()),
    )
    .unwrap();

    debug!("worker started");
    info!("processed {} jobs", 128);
    warn!("queue depth {}", 4096);
    error!("worker crashed");

    println!("--- captured ---");

    for line in memory.lines.lock().unwrap().iter() {
        println!("{line}");
    }
}
