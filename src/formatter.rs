use crate::Record;

pub trait Formatter {
    fn format(&self, record: &Record) -> String;
}
