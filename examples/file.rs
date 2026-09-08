use std::env;
use std::error::Error;
use std::fs;

use traccia::Config;
use traccia::Console;
use traccia::File;
use traccia::LevelFilter;
use traccia::debug;
use traccia::error;
use traccia::info;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::temp_dir().join("traccia-example.log");

    traccia::init(
        Config::empty()
            .with_min_level(LevelFilter::Debug)
            .with_target(Console::default())
            .with_target(File::new(&path, true)?),
    )
    .unwrap();

    debug!("opening connection pool");
    info!("migration {} applied", "0007_add_index");
    error!("job {} failed after {} retries", "reindex", 3);

    traccia::flush();

    println!("--- {} ---", path.display());
    print!("{}", fs::read_to_string(&path)?);

    Ok(())
}
