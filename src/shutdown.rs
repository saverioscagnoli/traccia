//! Simple wrapper around `atexit` to allow for a single function to be called when the program exits.
//!
//! Used for flushing logs and other cleanup tasks.

use std::ffi::c_int;

unsafe extern "C" {
    fn atexit(callback: extern "C" fn()) -> c_int;
}

pub fn add_hook(cb: extern "C" fn()) -> bool {
    let result: c_int;

    unsafe {
        result = atexit(cb);
    }

    result == 0
}
