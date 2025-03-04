//! Simple wrapper around `atexit` to allow for a single function to be called when the program exits.
//!
//! Used for flushing logs and other cleanup tasks.

use std::ffi::c_int;
use std::sync::OnceLock;

static HOOK: OnceLock<Option<Box<dyn Fn() + Send + Sync>>> = OnceLock::new();

extern "C" fn shutdown_wrapper() {
    if let Some(ref hook) = *HOOK.get().unwrap() {
        hook();
    }
}

unsafe extern "C" {
    fn atexit(callback: extern "C" fn()) -> c_int;
}

pub fn add_hook<F>(cb: F) -> bool
where
    F: Fn() + Send + Sync + 'static,
{
    let _ = HOOK.set(Some(Box::new(cb))).is_ok();

    let result: c_int;

    unsafe {
        result = atexit(shutdown_wrapper);
    }

    result == 0
}
