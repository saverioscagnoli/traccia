use traccia::{Color, Colorize, Style};

fn main() {
    println!(
        "{}",
        "This is yellow text on a cyan background!"
            .color(Color::Yellow)
            .background(Color::Cyan)
    );

    println!("{}", "This is bold text!".bold().color(Color::Red));
    println!("{}", "This is dim text!".dim());
    println!("{}", "This is italic text!".italic());
    println!("{}", "This is underlined text!".underline());
    println!(
        "{}",
        "This is a styled message!".bold().italic().underline()
    );
    println!("{}", "Blinking text!".blink());
    println!("{}", "Reversed background!".reverse());
    println!("{}", "Hidden text!".hidden());
    println!("{}", "Something wrong.".striketrough());

    println!("{}", "I like cyan".color(Color::BrightCyan));
}
