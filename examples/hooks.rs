use traccia::{Hook, info, warn};

fn main() {
    _ = traccia::set_hook(Hook::BeforeLog(Box::new(|| {
        println!("This is a before log hook");
    })));

    _ = traccia::set_hook(Hook::AfterLog(Box::new(|| {
        println!("This is an after log hook");
    })));

    traccia::init_default();

    info!("This is a test log message");
    warn!("This is a test warning message");
}
