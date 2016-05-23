//! Triggers

use std::error::Error;
use std::fmt;
use std::path::Path;

use LogFileInternals;

pub mod size;

/// Information about the active log file.
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
    /// Returns the path to the log file.
    pub fn path(&self) -> &Path {
        self.path
    }

    /// Returns an estimate of the log file's current size.
    ///
    /// This is calculated by taking the size of the log file when it is opened
    /// and adding the number of bytes written. It may be inaccurate if any
    /// writes have failed or if another process has modified the file
    /// concurrently.
    pub fn len(&self) -> u64 {
        self.len
    }
}

/// A trait which identifies if the active log file should be rolled over.
pub trait Trigger: fmt::Debug + Send + Sync + 'static {
    /// Determines if the active log file should be rolled over.
    fn trigger(&self, file: &LogFile) -> Result<bool, Box<Error>>;
}
