use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use traccia::Config;
use traccia::Level;
use traccia::LevelFilter;
use traccia::Target;

#[derive(Clone, Default)]
struct Capture(Arc<Mutex<Vec<String>>>);

impl Target for Capture {
    fn write(&self, _: Level, m: &str) -> Result<(), Box<dyn Error>> {
        self.0.lock().unwrap().push(m.to_string());
        Ok(())
    }
}

#[test]
fn boundaries() {
    let cap = Capture::default();

    traccia::init(
        Config::empty()
            .with_min_level(LevelFilter::Warn)
            .with_target(cap.clone())
            .with_module_filter("app::a", LevelFilter::Off)
            .with_module_filter("app::db", LevelFilter::Trace)
            .with_module_filter("app::db::pool", LevelFilter::Error),
    )
    .unwrap();

    log::error!(target: "app::abc", "sibling-not-silenced");
    log::error!(target: "app::a", "exact-silenced");
    log::error!(target: "app::a::inner", "child-silenced");
    log::trace!(target: "app::db", "exact-verbose");
    log::trace!(target: "app::db::conn", "child-verbose");
    log::trace!(target: "app::dbx", "sibling-not-verbose");
    log::trace!(target: "app::db::pool", "longest-prefix-wins");
    log::error!(target: "app::db::pool", "pool-error");
    log::warn!(target: "app", "root-warn");
    log::info!(target: "app", "root-info");

    let got = cap.0.lock().unwrap().clone();
    let got: Vec<&str> = got.iter().map(|s| s.rsplit(' ').next().unwrap()).collect();

    assert_eq!(
        got,
        vec![
            "sibling-not-silenced",
            "exact-verbose",
            "child-verbose",
            "pool-error",
            "root-warn",
        ]
    );
}
