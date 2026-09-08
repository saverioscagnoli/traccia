use traccia::Config;
use traccia::debug;
use traccia::error;
use traccia::info;
use traccia::trace;
use traccia::warn;

fn main() {
    traccia::init(Config::new()).unwrap();

    trace!("hidden at the default level");
    debug!("hidden at the default level");
    info!("listening on port {}", 8080);
    warn!("disk usage at {}%", 91);
    error!("could not reach {}", "db-primary");
}
