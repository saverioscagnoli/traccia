//! ANSI color codes for terminal output.
//!
//! Provides utilities for adding color to log messages in terminal output.
//! These colors are automatically stripped when writing to non-terminal
//! targets like files.

use std::fmt::Display;

/// Terminal colors for text output.
///
/// Represents the standard 8 ANSI colors available in most terminals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Black color (ANSI code 30 - bg: 40)
    Black,
    /// Red color (ANSI code 31 - bg: 41)
    Red,
    /// Green color (ANSI code 32 - bg: 42)
    Green,
    /// Yellow color (ANSI code 33 - bg: 43)
    Yellow,
    /// Blue color (ANSI code 34 - bg: 44)
    Blue,
    /// Magenta color (ANSI code 35 - bg: 45)
    Magenta,
    /// Cyan color (ANSI code 36 - bg: 46)
    Cyan,
    /// White color (ANSI code 37 - bg: 47)
    White,

    /// Default color (ANSI code 39)
    Default,

    /// Bright black color (ANSI code 90 - bg: 100)
    BrightBlack,

    /// Bright red color (ANSI code 91 - bg: 101)
    BrightRed,

    /// Bright green color (ANSI code 92 - bg: 102)
    BrightGreen,

    /// Bright yellow color (ANSI code 93 - bg: 103)
    BrightYellow,

    /// Bright blue color (ANSI code 94 - bg: 104)
    BrightBlue,

    /// Bright magenta color (ANSI code 95 - bg: 105)
    BrightMagenta,

    /// Bright cyan color (ANSI code 96 - bg: 106)
    BrightCyan,

    /// Bright white color (ANSI code 97 - bg: 107)
    BrightWhite,

    /// RGB color with 8-bit values for red, green, and blue components (https://gist.github.com/ConnerWill/d4b6c776b509add763e17f9f113fd25b#256-colors)
    ID(u8),

    /// RGB color with 8-bit values for red, green, and blue components.
    RGB(u8, u8, u8),
}

impl Color {
    /// Returns the ANSI escape code for the color.
    ///
    /// # Returns
    ///
    /// The ANSI escape sequence string for the color
    pub fn ansi_code_foreground(&self) -> String {
        match self {
            Color::Black => "\x1b[30m".to_string(),
            Color::Red => "\x1b[31m".to_string(),
            Color::Green => "\x1b[32m".to_string(),
            Color::Yellow => "\x1b[33m".to_string(),
            Color::Blue => "\x1b[34m".to_string(),
            Color::Magenta => "\x1b[35m".to_string(),
            Color::Cyan => "\x1b[36m".to_string(),
            Color::White => "\x1b[37m".to_string(),
            Color::Default => "\x1b[39m".to_string(),
            Color::BrightBlack => "\x1b[90m".to_string(),
            Color::BrightRed => "\x1b[91m".to_string(),
            Color::BrightGreen => "\x1b[92m".to_string(),
            Color::BrightYellow => "\x1b[93m".to_string(),
            Color::BrightBlue => "\x1b[94m".to_string(),
            Color::BrightMagenta => "\x1b[95m".to_string(),
            Color::BrightCyan => "\x1b[96m".to_string(),
            Color::BrightWhite => "\x1b[97m".to_string(),
            Color::ID(id) => format!("\x1b[38;5;{}m", id),
            Color::RGB(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
        }
    }

    pub fn ansi_code_background(&self) -> String {
        match self {
            Color::Black => "\x1b[40m".to_string(),
            Color::Red => "\x1b[41m".to_string(),
            Color::Green => "\x1b[42m".to_string(),
            Color::Yellow => "\x1b[43m".to_string(),
            Color::Blue => "\x1b[44m".to_string(),
            Color::Magenta => "\x1b[45m".to_string(),
            Color::Cyan => "\x1b[46m".to_string(),
            Color::White => "\x1b[47m".to_string(),
            Color::Default => "\x1b[49m".to_string(),
            Color::BrightBlack => "\x1b[100m".to_string(),
            Color::BrightRed => "\x1b[101m".to_string(),
            Color::BrightGreen => "\x1b[102m".to_string(),
            Color::BrightYellow => "\x1b[103m".to_string(),
            Color::BrightBlue => "\x1b[104m".to_string(),
            Color::BrightMagenta => "\x1b[105m".to_string(),
            Color::BrightCyan => "\x1b[106m".to_string(),
            Color::BrightWhite => "\x1b[107m".to_string(),
            Color::ID(id) => format!("\x1b[48;5;{}m", id),
            Color::RGB(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
        }
    }
}

/// Trait for applying colors to strings.
///
/// This trait provides the ability to color text using ANSI escape codes.
pub trait Colorize: Display {
    /// Applies a color to the string.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to apply
    ///
    /// # Returns
    ///
    /// A new string with the color applied via ANSI escape codes
    fn color(&self, color: Color) -> String {
        format!("{}{}\x1b[0m", color.ansi_code_foreground(), self)
    }

    /// Applies a background color to the string.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to apply
    ///
    /// # Returns
    ///
    /// A new string with the background color applied via ANSI escape codes
    fn background(&self, color: Color) -> String {
        format!("{}{}\x1b[0m", color.ansi_code_background(), self)
    }
}

impl Colorize for str {}
impl Colorize for String {}

pub trait Style: Display {
    /// Applies bold style to the string.
    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self)
    }

    /// Dims the color of the string.
    fn dim(&self) -> String {
        format!("\x1b[2m{}\x1b[0m", self)
    }

    /// Applies italic style to the string.
    fn italic(&self) -> String {
        format!("\x1b[3m{}\x1b[0m", self)
    }

    /// Underlines the string.
    fn underline(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", self)
    }

    /// Makes the content of the string blink.
    fn blink(&self) -> String {
        format!("\x1b[5m{}\x1b[0m", self)
    }

    /// Reverses the foreground and background colors of the string.
    fn reverse(&self) -> String {
        format!("\x1b[7m{}\x1b[0m", self)
    }

    /// Hides the content of the string with the background color.
    fn hidden(&self) -> String {
        format!("\x1b[8m{}\x1b[0m", self)
    }

    /// Strikes through the content of the string.
    fn striketrough(&self) -> String {
        format!("\x1b[9m{}\x1b[0m", self)
    }
}

impl Style for str {}
impl Style for String {}
