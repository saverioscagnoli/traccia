use traccia::Color;
use traccia::Colored;
use traccia::Style;

fn main() {
    println!("{}", "red".foreground_color(Color::Red));
    println!("{}", "bright cyan".foreground_color(Color::BrightCyan));
    println!("{}", "256-color 208".foreground_color(Color::Fixed(208)));
    println!(
        "{}",
        "true color".foreground_color(Color::Rgb(255, 105, 180))
    );
    println!("{}", "on blue".background_color(Color::Blue));

    println!("{}", "bold".bold());
    println!("{}", "dim".dim());
    println!("{}", "italic".italic());
    println!("{}", "underline".underline());
    println!("{}", "reverse".reverse());
    println!("{}", "strikethrough".strikethrough());

    println!("{}", "bold and green".foreground_color(Color::Green).bold());
    println!(
        "{}",
        format!("{} jobs queued", 12).foreground_color(Color::Yellow)
    );
}
