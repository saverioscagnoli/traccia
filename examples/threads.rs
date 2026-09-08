use std::thread;

use traccia::Config;
use traccia::LevelFilter;
use traccia::debug;
use traccia::info;
use traccia::warn;

fn main() {
    traccia::init(Config::new().with_min_level(LevelFilter::Debug)).unwrap();

    let handles: Vec<_> = (0..4)
        .map(|id| {
            thread::spawn(move || {
                debug!("worker {id} starting");

                for job in 0..3 {
                    info!("worker {id} finished job {job}");
                }

                warn!("worker {id} shutting down");
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    info!("all workers joined");
}
