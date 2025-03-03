/// ANSI color codes for terminal output.
///
/// Provides utilities for adding color to log messages in terminal output.
/// These colors are automatically stripped when writing to non-terminal
/// targets like files.

/// Terminal colors for text output.
///
/// Represents the standard 8 ANSI colors available in most terminals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Black color (ANSI code 30)
    Black,
    /// Red color (ANSI code 31)
    Red,
    /// Green color (ANSI code 32)
    Green,
    /// Yellow color (ANSI code 33)
    Yellow,
    /// Blue color (ANSI code 34)
    Blue,
    /// Magenta color (ANSI code 35)
    Magenta,
    /// Cyan color (ANSI code 36)
    Cyan,
    /// White color (ANSI code 37)
    White,
}

impl Color {
    /// Returns the ANSI escape code for the color.
    ///
    /// # Returns
    ///
    /// The ANSI escape sequence string for the color
    pub fn ansi_code(&self) -> &str {
        match self {
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
        }
    }
}

/// Trait for applying colors to strings.
///
/// This trait provides the ability to color text using ANSI escape codes.
pub trait Colorize {
    /// Applies a color to the string.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to apply
    ///
    /// # Returns
    ///
    /// A new string with the color applied via ANSI escape codes
    fn color(&self, color: Color) -> String;
}

impl Colorize for str {
    /// Applies a color to a string slice.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to apply
    ///
    /// # Returns
    ///
    /// A new string with the color applied
    ///
    /// # Examples
    ///
    /// ```
    /// use logger::{Color, Colorize};
    ///
    /// let colored = "Warning".color(Color::Yellow);
    /// ```
    fn color(&self, color: Color) -> String {
        format!("{}{}\x1b[0m", color.ansi_code(), self)
    }
}

impl Colorize for String {
    /// Applies a color to a String.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to apply
    ///
    /// # Returns
    ///
    /// A new string with the color applied
    ///
    /// # Examples
    ///
    /// ```
    /// use logger::{Color, Colorize};
    ///
    /// let message = String::from("Error occurred");
    /// let colored = message.color(Color::Red);
    /// ```
    fn color(&self, color: Color) -> String {
        format!("{}{}\x1b[0m", color.ansi_code(), self)
    }
}
