use traccia::Config;
use traccia::LevelFilter;
use traccia::debug;
use traccia::error;
use traccia::info;
use traccia::trace;
use traccia::warn;

fn main() {
    traccia::init(Config::new().with_min_level(LevelFilter::Trace)).unwrap();

    trace!("entering request handler");
    debug!("parsed {} headers", 12);
    info!("GET /users 200");
    warn!("retrying upstream call");
    error!("upstream returned 503");
}
