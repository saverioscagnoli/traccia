use std::borrow::Cow;
use std::env;
use std::error;
use std::io;
use std::io::IsTerminal;
use std::io::Write;

use crate::strings;
use crate::target::Target;

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
