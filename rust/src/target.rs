/// Target module defining output destinations for log messages.
use crate::{LogLevel, error::Error, util};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

/// Workaround to be able to clone boxed trait objects.
pub trait TargetClone {
    fn clone_box(&self) -> Box<dyn Target>;
}

impl<T> TargetClone for T
where
    T: 'static + Target + Clone,
{
    fn clone_box(&self) -> Box<dyn Target> {
        Box::new(self.clone())
    }
}

/// Represents the id of the target where the log message is written.
/// This can be a console output, file path, or a custom string identifier.
/// Useful for identifying the target in hooks / filters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TargetId {
    /// Console output target
    Console(Output),
    /// File output target
    File(PathBuf),
    /// Custom target identifier
    /// This can be used for custom targets that don't fit into the
    /// predefined categories.
    Custom(String),
}

/// Defines an output destination for log messages.
///
/// This trait allows the logger to write formatted messages to different
/// destinations such as console, files, or custom targets.
///
/// Implementors must be thread-safe (Send + Sync) and cloneable.
pub trait Target: Send + Sync + TargetClone {
    /// Writes a formatted log message to the target.
    ///
    /// # Arguments
    ///
    /// * `level` - The log level of the message
    /// * `formatted` - The formatted log message to write
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error if the write operation failed
    fn write(&self, level: LogLevel, formatted: &str) -> Result<(), Error>;

    /// Returns a custom filter level for the target.
    /// If the target has a filter level set, log messages with a lower
    /// level will be ignored.
    fn filter_level(&self) -> Option<LogLevel> {
        None
    }

    /// Returns the target ID for the target.
    /// This is used to identify the target in the logger.
    fn id(&self) -> TargetId {
        TargetId::Custom(format!("{:p}", self))
    }
}

impl Clone for Box<dyn Target> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Output destination for console log messages.
///
/// The default output is stdout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Output {
    /// Standard output (stdout)
    Stdout,
    /// Standard error (stderr)
    Stderr,
}

impl Default for Output {
    fn default() -> Self {
        Output::Stdout
    }
}

impl Default for &Output {
    fn default() -> Self {
        &Output::Stdout
    }
}

/// Standard console output target.
///
/// This target writes log messages to the standard output (stdout) or standard error (stderr)
/// using the Rust `println!` | `eprintln!` macro.
#[derive(Clone)]
pub struct Console {
    level: Option<LogLevel>,
    output: Option<Output>,
    filtered_outputs: Option<HashMap<LogLevel, Output>>,
}

impl Console {
    /// Creates a new console target
    pub fn new() -> Self {
        Console {
            level: None,
            output: None,
            filtered_outputs: None,
        }
    }

    /// Builder method to set the custom filter level for this target.
    pub fn filtered(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }

    /// Builder method to set the custom output for the console.
    /// This will write to the output for all the logs that target this console.
    ///
    /// If you want to set the output for a specific log level, use `filtered_outputs`.
    ///
    /// (e.g. You want to write all `LogLevel::Error` logs to stderr and all other logs to stdout)
    pub fn output(mut self, output: Output) -> Self {
        self.output = Some(output);
        self
    }

    /// Builder method to set the custom output for a specific log level.
    /// This behaves like the `output` method, but only applies to the specified log level.
    ///
    /// If you want to set the output for all logs, use `output`.
    /// Note: This will override the output set by `output`.
    ///
    /// (e.g. If you set `output` to `Stderr`, calling `filtered_outputs(LogLevel::Info, Output::Stdout)` will
    /// log all `LogLevel::Info` logs to stdout and all other logs to stderr)
    pub fn filtered_output(mut self, level: LogLevel, output: Output) -> Self {
        self.filtered_outputs
            .get_or_insert_with(HashMap::new)
            .insert(level, output);

        self
    }
}

impl Target for Console {
    /// Writes the formatted log message to the console.
    ///
    /// # Arguments
    ///
    /// * `formatted` - The formatted log message to write
    ///
    /// # Returns
    ///
    /// Always returns `Ok(())`
    fn write(&self, level: LogLevel, formatted: &str) -> Result<(), Error> {
        let output = self
            .filtered_outputs
            .as_ref()
            .and_then(|map| map.get(&level))
            .unwrap_or_else(|| self.output.as_ref().unwrap_or_default());

        match output {
            Output::Stdout => println!("{}", formatted),
            Output::Stderr => eprintln!("{}", formatted),
        }

        Ok(())
    }

    /// Returns the custom filter level for the console target.
    /// If the filter level is set, log messages with a lower level
    /// will be ignored.
    ///
    /// Useful for filtering log messages written to the console.
    fn filter_level(&self) -> Option<LogLevel> {
        self.level
    }

    /// Returns the target ID for the console target.
    /// This is used to identify the target in the logger.
    fn id(&self) -> TargetId {
        TargetId::Console(self.output.unwrap_or_default())
    }
}

/// File open mode for writing log messages.
/// - `Append`: Open the file in append mode, preserving existing content
/// - `Truncate`: Open the file in write mode, truncating existing content
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    Append,
    Truncate,
}

impl Default for FileMode {
    fn default() -> Self {
        FileMode::Append
    }
}

/// File output target.
///
/// This target writes log messages to a file on disk.
/// ANSI color codes are automatically stripped from messages written to files.
#[derive(Clone)]
pub struct File {
    path: PathBuf,
    inner: Arc<Mutex<fs::File>>,
    level: Option<LogLevel>,
}

impl Deref for File {
    type Target = Mutex<fs::File>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl File {
    /// Creates a new file target.
    ///
    /// This function will create the parent directories if they don't exist
    /// and open the file in append mode.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the log file'
    /// * `mode` - File open mode (default: `Append`)
    ///
    /// # Returns
    ///
    /// A new `File` target instance or an error if the file couldn't be opened
    ///
    /// # Examples
    ///
    /// ```
    /// use logger::{Config, File, init_with_config};
    ///
    /// let file_target = File::new("logs/app.log").expect("Failed to open log file");
    /// let config = Config {
    ///     targets: vec![Box::new(file_target)],
    ///     ..Config::default()
    /// };
    /// init_with_config(config);
    /// ```
    pub fn new<P>(path: P, mode: FileMode) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut options = OpenOptions::new();

        options.create(true);

        match mode {
            FileMode::Append => {
                options.append(true);
            }

            FileMode::Truncate => {
                options.write(true).truncate(true);
            }
        }

        let file = options.open(path)?;
        Ok(File {
            path: path.to_path_buf(),
            inner: Arc::new(Mutex::new(file)),
            level: None,
        })
    }

    /// Sets a custom filter level for the file target.
    /// If the filter level is set, log messages with a lower level
    /// will be ignored.
    ///
    /// Useful for filtering log messages written to files.
    pub fn filtered(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }
}

impl Target for File {
    /// Writes the formatted log message to the file.
    ///
    /// ANSI color codes are automatically stripped from the message
    /// before writing to the file.
    ///
    /// # Arguments
    ///
    /// * `formatted` - The formatted log message to write
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error if the write operation failed
    fn write(&self, _: LogLevel, formatted: &str) -> Result<(), Error> {
        let mut file = self.lock().map_err(|_| Error::Poisoned)?;
        let stripped = util::strip_ansi_codes(formatted);
        writeln!(file, "{}", stripped)?;
        Ok(())
    }

    /// Returns the custom filter level for the file target.
    /// If the filter level is set, log messages with a lower level
    /// will be ignored.
    fn filter_level(&self) -> Option<LogLevel> {
        self.level
    }

    /// Returns the target ID for the file target.
    /// This is used to identify the target in the logger.
    fn id(&self) -> TargetId {
        TargetId::File(self.path.clone())
    }
}
