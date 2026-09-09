use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A terminal color, either one of the sixteen named ones or an explicit value.
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    /// An index into the 256 color palette.
    Fixed(u8),

    /// A 24 bit color. Not every terminal honours these.
    Rgb(u8, u8, u8),
}

impl Color {
    /// The escape sequence that sets this as the text color.
    pub fn ansi_code_foreground(&self) -> String {
        match self {
            Self::Black => "\x1b[30m".to_string(),
            Self::Red => "\x1b[31m".to_string(),
            Self::Green => "\x1b[32m".to_string(),
            Self::Yellow => "\x1b[33m".to_string(),
            Self::Blue => "\x1b[34m".to_string(),
            Self::Magenta => "\x1b[35m".to_string(),
            Self::Cyan => "\x1b[36m".to_string(),
            Self::White => "\x1b[37m".to_string(),
            Self::Default => "\x1b[39m".to_string(),
            Self::BrightBlack => "\x1b[90m".to_string(),
            Self::BrightRed => "\x1b[91m".to_string(),
            Self::BrightGreen => "\x1b[92m".to_string(),
            Self::BrightYellow => "\x1b[93m".to_string(),
            Self::BrightBlue => "\x1b[94m".to_string(),
            Self::BrightMagenta => "\x1b[95m".to_string(),
            Self::BrightCyan => "\x1b[96m".to_string(),
            Self::BrightWhite => "\x1b[97m".to_string(),
            Self::Fixed(id) => format!("\x1b[38;5;{}m", id),
            Self::Rgb(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
        }
    }

    /// The escape sequence that sets this as the background color.
    pub fn ansi_code_background(&self) -> String {
        match self {
            Self::Black => "\x1b[40m".to_string(),
            Self::Red => "\x1b[41m".to_string(),
            Self::Green => "\x1b[42m".to_string(),
            Self::Yellow => "\x1b[43m".to_string(),
            Self::Blue => "\x1b[44m".to_string(),
            Self::Magenta => "\x1b[45m".to_string(),
            Self::Cyan => "\x1b[46m".to_string(),
            Self::White => "\x1b[47m".to_string(),
            Self::Default => "\x1b[49m".to_string(),
            Self::BrightBlack => "\x1b[100m".to_string(),
            Self::BrightRed => "\x1b[101m".to_string(),
            Self::BrightGreen => "\x1b[102m".to_string(),
            Self::BrightYellow => "\x1b[103m".to_string(),
            Self::BrightBlue => "\x1b[104m".to_string(),
            Self::BrightMagenta => "\x1b[105m".to_string(),
            Self::BrightCyan => "\x1b[106m".to_string(),
            Self::BrightWhite => "\x1b[107m".to_string(),
            Self::Fixed(id) => format!("\x1b[48;5;{}m", id),
            Self::Rgb(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
        }
    }
}

/// Colors anything that can be displayed.
///
/// ```
/// use traccia::{Color, Colored};
///
/// let line = format!("{} jobs", 12).foreground_color(Color::Yellow);
/// ```
pub trait Colored: fmt::Display {
    /// Wraps the value in a color and a reset.
    fn foreground_color(&self, color: Color) -> String {
        format!("{}{}\x1b[0m", color.ansi_code_foreground(), self)
    }

    /// Wraps the value in a background color and a reset.
    fn background_color(&self, color: Color) -> String {
        format!("{}{}\x1b[0m", color.ansi_code_background(), self)
    }
}

impl<T> Colored for T where T: fmt::Display {}

/// Bold, italic and the rest, on anything that can be displayed.
///
/// These return `String`, so they chain with [`Colored`].
///
/// ```
/// use traccia::{Color, Colored, Style};
///
/// let line = "ready".foreground_color(Color::Green).bold();
/// ```
pub trait Style: fmt::Display {
    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self)
    }

    fn dim(&self) -> String {
        format!("\x1b[2m{}\x1b[0m", self)
    }

    fn italic(&self) -> String {
        format!("\x1b[3m{}\x1b[0m", self)
    }

    fn underline(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", self)
    }

    fn blink(&self) -> String {
        format!("\x1b[5m{}\x1b[0m", self)
    }

    fn reverse(&self) -> String {
        format!("\x1b[7m{}\x1b[0m", self)
    }

    fn hidden(&self) -> String {
        format!("\x1b[8m{}\x1b[0m", self)
    }

    fn strikethrough(&self) -> String {
        format!("\x1b[9m{}\x1b[0m", self)
    }
}

impl<T: fmt::Display> Style for T {}

/// Turns a record into the line that targets write.
///
/// It runs once per record and every target sees the same string, so put color in here
/// and let the target strip it when it is not wanted.
pub trait Formatter: Send + Sync {
    /// Builds the line. Ends without a newline; targets add their own.
    fn format(&self, record: &log::Record) -> String;
}

/// `[INFO] [module] message`, with the level in color and the module dimmed.
pub struct DefaultFormatter;

impl Formatter for DefaultFormatter {
    fn format(&self, record: &log::Record) -> String {
        let (name, color) = match record.level() {
            log::Level::Trace => ("TRCE", Color::Cyan),
            log::Level::Debug => ("DEBG", Color::Magenta),
            log::Level::Info => ("INFO", Color::Green),
            log::Level::Warn => ("WARN", Color::Yellow),
            log::Level::Error => ("ERRO", Color::Red),
        };

        let level = format!("[{}]", name).foreground_color(color);
        let target = format!("[{}]", record.target()).foreground_color(Color::BrightBlack);

        format!("{} {} {}", level, target, record.args())
    }
}
