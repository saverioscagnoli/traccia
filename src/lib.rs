mod color;
mod format;
mod util;

pub use color::{Color, Colorize};

use format::default_format;
use std::{
    fmt::{Debug, Display},
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
        mpsc::{self},
    },
    thread::JoinHandle,
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = match self {
            LogLevel::Error => "ERROR".color(Color::Red),
            LogLevel::Warn => "WARN".color(Color::Yellow),
            LogLevel::Info => "INFO".color(Color::Green),
            LogLevel::Debug => "DEBUG".color(Color::Blue),
            LogLevel::Trace => "TRACE".color(Color::Magenta),
        };

        write!(f, "{}", level)
    }
}

static ABORT: AtomicBool = AtomicBool::new(false);

fn abort() -> bool {
    ABORT.load(Ordering::Relaxed)
}

fn set_abort(value: bool) {
    ABORT.store(value, Ordering::Relaxed);
}

pub struct Logger {
    sender: mpsc::Sender<(LogLevel, String)>,
    handle: Mutex<Option<JoinHandle<()>>>,
}

impl Logger {
    fn new<F>(level: LogLevel, file: Option<PathBuf>, format: F) -> Self
    where
        F: Fn(LogLevel, &str) -> String + Send + Sync + 'static,
    {
        let (sender, receiver) = mpsc::channel::<(LogLevel, String)>();

        let handle = std::thread::spawn(move || {
            Self::logging_thread(receiver, level, Box::new(format), file)
        });

        Self {
            sender,
            handle: Mutex::new(Some(handle)),
        }
    }

    fn handle_logging(
        rx_level: LogLevel,
        env_level: LogLevel,
        message: &str,
        format: &dyn Fn(LogLevel, &str) -> String,
        file: &mut Option<std::fs::File>,
    ) {
        if rx_level > env_level {
            return;
        }

        let formatted = format(rx_level, &message);
        print!("{}", formatted);

        if let Some(file) = file {
            let stripped = util::strip_ansi_codes(&formatted);
            file.write_all(stripped.as_bytes())
                .expect("Failed to write to log file");
        }
    }

    fn logging_thread(
        rx: mpsc::Receiver<(LogLevel, String)>,
        env_level: LogLevel,
        format: Box<dyn Fn(LogLevel, &str) -> String + Send + Sync + 'static>,
        file: Option<PathBuf>,
    ) {
        if let Some(ref file) = file {
            if let Some(parent) = file.parent() {
                std::fs::create_dir_all(parent).expect("Failed to create log directory");
            }
        }

        let mut file = file.as_ref().map(|path| {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .expect("Failed to open log file")
        });

        while !abort() {
            if let Ok((rx_level, message)) = rx.recv_timeout(Duration::from_millis(100)) {
                Self::handle_logging(rx_level, env_level, &message, &format, &mut file);
            }
        }

        // Process remaining messages in the channel
        while let Ok((rx_level, message)) = rx.try_recv() {
            Self::handle_logging(rx_level, env_level, &message, &format, &mut file);
        }
    }

    fn log(&self, level: LogLevel, message: &str) {
        self.sender
            .send((level, message.to_string()))
            .expect("Failed to send log message");
    }
}

static LOGGER: OnceLock<Logger> = OnceLock::new();
static FORMAT: OnceLock<Box<dyn Fn(LogLevel, &str) -> String + Send + Sync + 'static>> =
    OnceLock::new();
static CLEAR_ON_START: AtomicBool = AtomicBool::new(false);

pub fn init(level: LogLevel) {
    let format = FORMAT.get_or_init(|| Box::new(default_format));

    LOGGER
        .set(Logger::new(level, None, format))
        .unwrap_or_else(|_| panic!("A logger has already been initialized"));
}

pub fn init_with_file<P>(level: LogLevel, file: P)
where
    P: Into<PathBuf>,
{
    let format = FORMAT.get_or_init(|| Box::new(default_format));

    LOGGER
        .set(Logger::new(level, Some(file.into()), format))
        .unwrap_or_else(|_| panic!("A logger has already been initialized"));
}

pub fn format<F>(format: F)
where
    F: Fn(LogLevel, &str) -> String + Send + Sync + 'static,
{
    FORMAT
        .set(Box::new(format))
        .unwrap_or_else(|_| panic!("A format has already been set"));
}

pub fn clear_on_start() {
    CLEAR_ON_START.store(true, Ordering::Relaxed);
}

pub fn shutdown() {
    set_abort(true);
    if let Some(logger) = LOGGER.get() {
        if let Ok(mut handle) = logger.handle.lock() {
            if let Some(handle) = handle.take() {
                handle.join().ok();
            }
        }
    }
}

pub fn log(level: LogLevel, message: &str) {
    if let Some(logger) = LOGGER.get() {
        logger.log(level, message);
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log(LogLevel::Error, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log(LogLevel::Warn, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Info, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log(LogLevel::Debug, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log(LogLevel::Trace, &format!($($arg)*));
    };
}

