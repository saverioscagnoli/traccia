/// Target module defining output destinations for log messages.
use crate::{LogLevel, error::Error, util};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    ops::Deref,
    path::Path,
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
    /// * `formatted` - The formatted log message to write
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error if the write operation failed
    fn write(&self, formatted: &str) -> Result<(), Error>;

    fn custom_level(&self) -> Option<LogLevel> {
        None
    }
}

impl Clone for Box<dyn Target> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Standard console output target.
///
/// This target writes log messages to the standard output (stdout)
/// using the Rust `println!` macro.
#[derive(Clone)]
pub struct Console {
    level: Option<LogLevel>,
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
    fn write(&self, formatted: &str) -> Result<(), Error> {
        println!("{}", formatted);
        Ok(())
    }

    fn custom_level(&self) -> Option<LogLevel> {
        self.level
    }
}

impl Console {
    pub fn new() -> Self {
        Console { level: None }
    }

    pub fn new_filtered(level: LogLevel) -> Self {
        Console { level: Some(level) }
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
    inner: Arc<Mutex<fs::File>>,
    level: Option<LogLevel>,
}

impl Deref for File {
    type Target = Mutex<fs::File>;

    fn deref(&self) -> &Self::Target {
        &self.inner
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
    fn write(&self, formatted: &str) -> Result<(), Error> {
        let mut file = self.lock().map_err(|_| Error::Poisoned)?;
        let stripped = util::strip_ansi_codes(formatted);
        writeln!(file, "{}", stripped)?;
        Ok(())
    }

    fn custom_level(&self) -> Option<LogLevel> {
        self.level
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
            inner: Arc::new(Mutex::new(file)),
            level: None,
        })
    }

    pub fn new_filtered<P>(path: P, mode: FileMode, level: LogLevel) -> Result<Self, Error>
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
            inner: Arc::new(Mutex::new(file)),
            level: Some(level),
        })
    }
}
