use alloc::boxed::Box;
use core::error;

/// Somewhere formatted lines end up.
///
/// `write` receives the level as well as the text, so a target can ignore records it
/// does not care about. Errors are not propagated to the caller of `info!` and friends;
/// the logger prints the line to stderr instead and carries on, or drops it when the
/// `std` feature is off.
pub trait Target: Send + Sync {
    /// Writes one already formatted line.
    fn write(&self, level: log::Level, message: &str) -> Result<(), Box<dyn error::Error>>;

    /// Pushes anything buffered. Called by `log::logger().flush()` and by `fatal!`.
    fn flush(&self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
}
