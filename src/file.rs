use std::error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::sync;

use crate::strings;
use crate::target::Target;

/// Appends lines to a file.
///
/// Escape sequences are stripped on the way out, so colored formatters still produce a
/// readable file.
pub struct File {
    mu: sync::Mutex<fs::File>,
}

impl File {
    /// Opens `path`, creating it if needed. `clean` truncates it first instead of
    /// appending.
    pub fn new<P>(path: P, clean: bool) -> Result<Self, io::Error>
    where
        P: AsRef<Path>,
    {
        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(clean)
            .append(!clean)
            .create(true)
            .open(path)?;

        Ok(Self {
            mu: sync::Mutex::new(file),
        })
    }
}

impl Target for File {
    fn write(&self, _: log::Level, message: &str) -> Result<(), Box<dyn error::Error>> {
        let mut file = self.mu.lock().unwrap_or_else(|e| e.into_inner());
        let message = strings::strip_ansi(message);

        writeln!(file, "{message}")?;

        Ok(())
    }

    fn flush(&self) -> Result<(), Box<dyn error::Error>> {
        self.mu.lock().unwrap_or_else(|e| e.into_inner()).flush()?;
        Ok(())
    }
}
