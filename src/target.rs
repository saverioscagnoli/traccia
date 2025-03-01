use dyn_clone::DynClone;

use crate::{error::Error, util};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    ops::Deref,
    path::Path,
    sync::{Arc, Mutex},
};

pub trait Target: Send + Sync + DynClone {
    fn write(&self, formatted: &str) -> Result<(), Error>;
}

dyn_clone::clone_trait_object!(Target);

#[derive(Clone)]
pub struct Console;

impl Target for Console {
    fn write(&self, formatted: &str) -> Result<(), Error> {
        println!("{}", formatted);
        Ok(())
    }
}

#[derive(Clone)]
pub struct File(Arc<Mutex<fs::File>>);

impl Deref for File {
    type Target = Mutex<fs::File>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl File {
    pub fn new<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(File(Arc::new(Mutex::new(file))))
    }
}

impl Target for File {
    fn write(&self, formatted: &str) -> Result<(), Error> {
        let mut file = self.lock().map_err(|_| Error::Poisoned)?;
        let stripped = util::strip_ansi_codes(formatted);
        writeln!(file, "{}", stripped)?;
        Ok(())
    }
}
