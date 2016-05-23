//! Rollers

use std::error::Error;
use std::fmt;
use std::path::Path;

pub mod delete;
pub mod fixed_window;

/// A trait which processes log files after they have been rolled over.
pub trait Roll: fmt::Debug + Send + Sync + 'static {
    /// Processes the log file.
    ///
    /// At the time that this method has been called, the log file has already
    /// been closed.
    ///
    /// If this method returns successfully, there *must* no longer be a file
    /// at the specified location.
    fn roll(&self, file: &Path) -> Result<(), Box<Error>>;
}
