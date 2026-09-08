use traccia::Color;
use traccia::Colored;
use traccia::Config;
use traccia::Formatter;
use traccia::Level;
use traccia::LevelFilter;
use traccia::Record;
use traccia::Style;
use traccia::debug;
use traccia::error;
use traccia::info;
use traccia::warn;

struct Compact;

impl Formatter for Compact {
    fn format(&self, record: &Record) -> String {
        let (glyph, color) = match record.level() {
            Level::Trace => ("·", Color::BrightBlack),
            Level::Debug => ("›", Color::Blue),
            Level::Info => ("✓", Color::Green),
            Level::Warn => ("!", Color::Yellow),
            Level::Error => ("✗", Color::Red),
        };

        let origin = match (record.file(), record.line()) {
            (Some(file), Some(line)) => format!("{file}:{line}"),
            _ => record.target().to_string(),
        };

        format!(
            "{} {} {}",
            glyph.foreground_color(color).bold(),
            record.args(),
            origin.foreground_color(Color::BrightBlack).dim(),
        )
    }
}

fn main() {
    traccia::init(
        Config::new()
            .with_min_level(LevelFilter::Debug)
            .with_formatter(Compact),
    )
    .unwrap();

    debug!("cache warm");
    info!("checkout completed for order {}", 41822);
    warn!("payment provider slow: {}ms", 1840);
    error!("refund failed");
}
