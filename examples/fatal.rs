use traccia::Config;
use traccia::fatal;
use traccia::info;

fn load_config(path: &str) -> Option<String> {
    let _ = path;
    None
}

fn main() {
    traccia::init(Config::new()).unwrap();

    info!("starting up");

    let Some(config) = load_config("/etc/app/config.toml") else {
        fatal!(code: 78, "could not read {}", "/etc/app/config.toml");
    };

    info!("loaded {config}");
}
