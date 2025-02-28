use traccia::{Color, Colorize};

fn main() {
    println!("{}", "This is a warning. ⚠️".color(Color::Yellow));
    println!("{}", "This is an error! ⛔".color(Color::Red));
    println!("{}", "This is a success! 🥳".color(Color::Green));
}
