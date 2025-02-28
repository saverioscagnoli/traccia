use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    NotInitialized,
    AlreadyInitialized,
    Poisoned,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::NotInitialized => write!(f, "A logger has not been initialized"),
            Error::AlreadyInitialized => write!(f, "A logger has already been initialized"),
            Error::Poisoned => write!(f, "Mutex is poisoned"),
        }
    }
}

impl std::error::Error for Error {}
