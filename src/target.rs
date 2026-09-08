use std::borrow::Cow;
use std::env;
use std::error;
use std::fs;
use std::io;
use std::io::IsTerminal;
use std::io::Write;
use std::path::Path;
use std::sync;

use crate::strings;

/// Somewhere formatted lines end up.
///
/// `write` receives the level as well as the text, so a target can ignore records it
/// does not care about. Errors are not propagated to the caller of `info!` and friends;
/// the logger prints the line to stderr instead and carries on.
pub trait Target: Send + Sync {
    /// Writes one already formatted line.
    fn write(&self, level: log::Level, message: &str) -> Result<(), Box<dyn error::Error>>;

    /// Pushes anything buffered. Called by `log::logger().flush()` and by `fatal!`.
    fn flush(&self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
}

#[derive(Default)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Which of the two standard streams a [`Console`] writes to.
pub enum Output {
    Stdout,
    #[default]
    Stderr,
}

impl Output {
    fn is_terminal(self) -> bool {
        match self {
            Self::Stdout => io::stdout().is_terminal(),
            Self::Stderr => io::stderr().is_terminal(),
        }
    }
}

/// Color unless `NO_COLOR` is set to something, or the stream is not a terminal.
fn detect_color(output: Output) -> bool {
    match env::var_os("NO_COLOR") {
        Some(v) if !v.is_empty() => false,
        _ => output.is_terminal(),
    }
}

#[derive(Debug, Clone, Copy)]
/// Writes to stdout or stderr.
///
/// Color is decided once, when you build it, so there is no `isatty` call per line.
pub struct Console {
    output: Output,
    colored: bool,
}

impl Default for Console {
    fn default() -> Self {
        Self::auto(Output::default())
    }
}

impl Console {
    /// Picks the stream and the color yourself, skipping detection.
    pub fn new(out: Output, colored: bool) -> Self {
        Self {
            output: out,
            colored,
        }
    }

    /// Picks the stream, and turns color on only if it is a terminal and `NO_COLOR`
    /// is unset.
    pub fn auto(out: Output) -> Self {
        Self {
            output: out,
            colored: detect_color(out),
        }
    }

    /// Overrides whatever was detected.
    pub fn with_color(mut self, color: bool) -> Self {
        self.colored = color;
        self
    }
}

impl Target for Console {
    fn write(&self, _: log::Level, message: &str) -> Result<(), Box<dyn error::Error>> {
        let message = if self.colored {
            Cow::Borrowed(message)
        } else {
            Cow::Owned(strings::strip_ansi(message))
        };

        match self.output {
            Output::Stdout => writeln!(io::stdout(), "{message}")?,
            Output::Stderr => writeln!(io::stderr(), "{message}")?,
        }

        Ok(())
    }
}

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
