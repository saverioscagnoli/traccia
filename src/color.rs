pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
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

pub trait Colorize {
    fn color(&self, color: Color) -> String;
}

impl Colorize for str {
    fn color(&self, color: Color) -> String {
        format!("{}{}\x1b[0m", color.ansi_code(), self)
    }
}

impl Colorize for String {
    fn color(&self, color: Color) -> String {
        format!("{}{}\x1b[0m", color.ansi_code(), self)
    }
}
