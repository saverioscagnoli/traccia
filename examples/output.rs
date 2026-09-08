use traccia::Config;
use traccia::Console;
use traccia::LevelFilter;
use traccia::Output;
use traccia::info;
use traccia::warn;

fn main() {
    traccia::init(
        Config::empty()
            .with_min_level(LevelFilter::Debug)
            .with_target(Console::new(Output::Stdout, true)),
    )
    .unwrap();

    info!("stdout keeps logs out of the error stream");
    warn!("pipe with `cargo run --example output | cat` to see it");
}
