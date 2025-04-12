use std::{fmt::Display, io, sync::PoisonError};

#[derive(Debug)]
pub enum Error {
    /// Generic wrapper over io::Error
    Io(io::Error),
    /// The logger requires to be initialized but it isn't
    NotInitialized,
    /// The logger has been initialized more than once.
    AlreadyInitialized,
    /// The mutex is poisoned (i.e. `File` targets)
    /// Or the `HOOKS` RwLock is poisoned
    Poisoned,
    /// Failed to convert `LogLevel` to something else or vice-versa
    ParseLogLevel,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Error::Poisoned
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::NotInitialized => write!(f, "A logger has not been initialized"),
            Error::AlreadyInitialized => write!(f, "A logger has already been initialized"),
            Error::Poisoned => write!(f, "Mutex is poisoned"),
            Error::ParseLogLevel => write!(f, "Could not parse log level from string"),
        }
    }
}

impl std::error::Error for Error {}
