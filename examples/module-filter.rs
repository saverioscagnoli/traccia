use traccia::Config;
use traccia::LevelFilter;

mod db {
    pub fn query() {
        traccia::trace!("SELECT * FROM users WHERE id = $1");
        traccia::debug!("query took {}ms", 4);
    }
}

mod http {
    pub fn serve() {
        traccia::trace!("hidden: http is capped at info");
        traccia::debug!("hidden: http is capped at info");
        traccia::info!("GET /users 200");
    }
}

fn main() {
    traccia::init(
        Config::new()
            .with_min_level(LevelFilter::Warn)
            .with_module_filter("module_filter::db", LevelFilter::Trace)
            .with_module_filter("module_filter::http", LevelFilter::Info),
    )
    .unwrap();

    db::query();
    http::serve();

    traccia::info!("hidden: the crate root is capped at warn");
    traccia::warn!("shown: the crate root is capped at warn");
}
