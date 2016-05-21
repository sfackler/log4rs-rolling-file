use std::error::Error;
use std::fmt;
use std::path::Path;

use LogFileInternals;

pub mod size;

pub struct LogFile<'a> {
    path: &'a Path,
    len: u64,
}

impl<'a> LogFileInternals<'a> for LogFile<'a> {
    fn new(path: &'a Path, len: u64) -> LogFile<'a> {
        LogFile {
            path: path,
            len: len,
        }
    }
}

impl<'a> LogFile<'a> {
    pub fn path(&self) -> &Path {
        self.path
    }

    pub fn len(&self) -> u64 {
        self.len
    }
}

pub trait Trigger: fmt::Debug + Send + Sync + 'static {
    fn trigger(&self, file: &LogFile) -> Result<bool, Box<Error>>;
}
