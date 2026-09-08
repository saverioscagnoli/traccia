//! A small logging framework. Uses `log`
//!
//! Set it up once with [`init`] and then use the `log` macros as usual. Anything in
//! your dependency tree that logs through `log` ends up here too.
//!
//! ```
//! use traccia::{Config, info};
//!
//! traccia::init(Config::new()).unwrap();
//! info!("listening on port {}", 8080);
//! ```
//!
//! [`Config`] decides the level, where lines go and how they look. Implement [`Target`]
//! to send them somewhere other than the console or a file, and [`Formatter`] to change
//! their shape.

mod strings;
mod style;
mod target;

use std::sync::OnceLock;

pub use log::Level;
pub use log::LevelFilter;
pub use log::Record;
pub use log::SetLoggerError;
pub use log::debug;
pub use log::error;
pub use log::info;
pub use log::trace;
pub use log::warn;

pub use crate::strings::strip_ansi;
pub use crate::style::Color;
pub use crate::style::Colored;
pub use crate::style::DefaultFormatter;
pub use crate::style::Formatter;
pub use crate::style::Style;
pub use crate::target::Console;
pub use crate::target::File;
pub use crate::target::Output;
pub use crate::target::Target;

/// The logger traccia hands to the `log` facade.
///
/// Built from a [`Config`] by [`init`]. You should not need to name it.
pub struct Logger {
    min_level: log::LevelFilter,
    targets: Vec<Box<dyn Target>>,
    formatter: Box<dyn Formatter>,
    module_filters: Vec<(String, log::LevelFilter)>,
}

impl Logger {
    fn emit(&self, record: &log::Record) {
        let message = self.formatter.format(record);

        for t in &self.targets {
            if t.write(record.level(), &message).is_err() {
                eprintln!("{}: {}", record.level().as_str(), message);
            }
        }
    }
}

fn module_matches(target: &str, prefix: &str) -> bool {
    match target.strip_prefix(prefix) {
        Some("") => true,
        Some(rest) => rest.starts_with("::"),
        None => false,
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let target = metadata.target();
        let level = self
            .module_filters
            .iter()
            .filter(|(prefix, _)| module_matches(target, prefix))
            .max_by_key(|(prefix, _)| prefix.len())
            .map(|(_, lvl)| *lvl)
            .unwrap_or(self.min_level);

        metadata.level() <= level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        self.emit(record);
    }

    fn flush(&self) {
        for t in &self.targets {
            if t.flush().is_err() {
                eprintln!("Failed to flush target");
            }
        }
    }
}

/// How the logger behaves.
///
/// [`Config::new`] gives you stderr at the info level. [`Config::empty`] gives you the
/// same thing with no targets, so you can choose them yourself.
///
/// ```no_run
/// use traccia::{Config, Console, File, LevelFilter, Output};
///
/// traccia::init(
///     Config::empty()
///         .with_min_level(LevelFilter::Debug)
///         .with_target(Console::new(Output::Stdout, true))
///         .with_target(File::new("app.log", true)?)
///         .with_module_filter("myapp::db", LevelFilter::Trace),
/// )?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Config {
    /// Records below this level are dropped, unless a module filter says otherwise.
    pub min_level: log::LevelFilter,

    /// Every target gets every line that survives filtering.
    pub targets: Vec<Box<dyn Target>>,

    /// Runs once per record. Targets share the result.
    pub formatter: Box<dyn Formatter>,

    /// Per module level overrides, longest matching prefix first.
    pub module_filters: Vec<(String, log::LevelFilter)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            min_level: log::LevelFilter::Info,
            targets: vec![Box::new(Console::default())],
            formatter: Box::new(DefaultFormatter),
            module_filters: Vec::new(),
        }
    }
}

impl Config {
    /// Stderr, at the info level.
    pub fn new() -> Self {
        Self::default()
    }

    /// Info level with no targets at all. Pair it with [`with_target`](Self::with_target).
    pub fn empty() -> Self {
        Self {
            min_level: log::LevelFilter::Info,
            targets: Vec::new(),
            formatter: Box::new(DefaultFormatter),
            module_filters: Vec::new(),
        }
    }

    /// Sets the level below which records are thrown away.
    pub fn with_min_level(mut self, level: log::LevelFilter) -> Self {
        self.min_level = level;
        self
    }

    /// Adds a target, keeping the ones already there.
    pub fn with_target<T>(mut self, target: T) -> Self
    where
        T: Target + 'static,
    {
        self.targets.push(Box::new(target));
        self
    }

    /// Replaces the whole target list.
    pub fn with_targets<I>(mut self, targets: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn Target>>,
    {
        self.targets = targets.into_iter().collect();
        self
    }

    /// Sets the formatter. The default is [`DefaultFormatter`].
    pub fn with_formatter<F>(mut self, formatter: F) -> Self
    where
        F: Formatter + 'static,
    {
        self.formatter = Box::new(formatter);
        self
    }

    /// Overrides the level for a module and everything under it.
    ///
    /// The longest matching prefix wins, so a filter on `myapp::db` beats one on
    /// `myapp`. Prefixes match on module boundaries, so `myapp` does not catch
    /// `myapp_extra`.
    pub fn with_module_filter<T>(mut self, module: T, level: log::LevelFilter) -> Self
    where
        T: Into<String>,
    {
        self.module_filters.push((module.into(), level));
        self
    }

    fn build_logger(self) -> Logger {
        Logger {
            min_level: self.min_level,
            targets: self.targets,
            formatter: self.formatter,
            module_filters: self.module_filters,
        }
    }
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

/// Installs the logger.
///
/// Only the first call takes effect. Later ones leave the original config alone and
/// return an error, which is also what you get if something else already claimed the
/// `log` facade.
pub fn init(config: Config) -> Result<(), log::SetLoggerError> {
    let logger = LOGGER.get_or_init(|| config.build_logger());
    let max = std::iter::once(logger.min_level)
        .chain(logger.module_filters.iter().map(|(_, l)| *l))
        .max()
        .unwrap_or(logger.min_level);

    log::set_logger(logger)?;
    log::set_max_level(max);

    Ok(())
}

/// Flushes every target.
///
/// Only the file target buffers anything, and `fatal!` already does this for you.
pub fn flush() {
    log::logger().flush();
}

#[doc(hidden)]
pub fn __fatal(args: std::fmt::Arguments, target: &str, file: &str, line: u32, code: i32) -> ! {
    match LOGGER.get() {
        Some(logger) => {
            logger.emit(
                &log::Record::builder()
                    .args(args)
                    .level(log::Level::Error)
                    .target(target)
                    .module_path(Some(target))
                    .file(Some(file))
                    .line(Some(line))
                    .build(),
            );
            log::Log::flush(logger);
        }
        None => eprintln!("fatal: {args}"),
    }

    std::process::exit(code)
}

/// Logs at the error level, flushes every target, then exits the process.
///
/// ```no_run
/// # use traccia::fatal;
/// fatal!("could not reach {}", "db-primary");
/// fatal!(code: 78, "bad config");
/// ```
///
/// The exit code is 1 unless you pass `code:`.
#[macro_export]
macro_rules! fatal {
    (code: $code:expr, $($arg:tt)*) => {
        $crate::__fatal(
            ::core::format_args!($($arg)*),
            ::core::module_path!(),
            ::core::file!(),
            ::core::line!(),
            $code,
        )
    };
    ($($arg:tt)*) => { $crate::fatal!(code: 1, $($arg)*) };
}
